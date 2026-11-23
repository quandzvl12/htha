//! BƯỚC 4a — luật chiến đấu cốt lõi, port từ `spawnEnemy`, `applyClass`, `gainXP`, `killEnemy`, vòng lặp quái.
//! Mọi giá trị ngẫu nhiên được truyền vào tường minh (`SpawnRolls`) để đối chiếu từng con số với JS.

use crate::balance::{self, js_round, SPEED_CAP, UP_PCT};
use crate::data_gen::{ClassDef, EnemyProfile, CLASSES, ENEMY_PROFILES};
use crate::dungeon::STAGES_PER_DUNGEON;

pub const WW: f64 = 2600.0;
pub const WH: f64 = 2600.0;
pub const TAU: f64 = std::f64::consts::TAU;
pub const ENEMY_ORDER: [&str; 7] = ["duc", "tayhung", "quan", "duy", "tam", "nghia", "thanh"];
pub const PLAYER_R: f64 = 18.0;

pub fn enemy_profile(key: &str) -> Option<&'static EnemyProfile> {
    ENEMY_PROFILES.iter().find(|p| p.key == key)
}
pub fn class_def(key: &str) -> Option<&'static ClassDef> {
    CLASSES.iter().find(|c| c.key == key)
}

/// `Math.min(26, 8 + Math.floor(stage*.65))`
pub fn stage_enemy_count(stage: i64) -> i64 {
    (8 + (stage as f64 * 0.65).floor() as i64).min(26)
}
/// `stage % STAGES_PER_DUNGEON === 0`
pub fn is_dungeon_boss_stage(stage: i64) -> bool {
    stage % STAGES_PER_DUNGEON as i64 == 0
}

/// Các lần `Math.random()` mà `spawnEnemy` tiêu thụ, theo đúng thứ tự: góc, khoảng cách, lastAttack, (tinh anh).
#[derive(Clone, Copy, Debug)]
pub struct SpawnRolls {
    pub angle_u: f64,
    pub dist_u: f64,
    pub last_attack_u: f64,
    pub elite_u: f64,
}

#[derive(Clone, Debug)]
pub struct EnemyInit {
    pub kind: &'static str,
    pub name: &'static str,
    pub color: &'static str,
    pub hp: f64,
    pub max_hp: f64,
    pub damage: f64,
    pub rate: f64,
    pub r: f64,
    pub speed: f64,
    pub is_boss: bool,
    pub hidden_boss: bool,
    pub boss_tier: i64,
    pub damage_taken_mult: f64,
    pub damage_out_mult: f64,
    pub freeze_resist: bool,
    pub reflect: bool,
    pub lifesteal: f64,
    pub elite: bool,
    pub x: f64,
    pub y: f64,
    pub last_attack: f64,
    pub skill_timer: f64,
    pub teleport_timer: f64,
    pub time_stop_timer: f64,
    pub black_hole_timer: f64,
}

/// `forced` = `forcedType` (boss ẩn Tày Hùng / Thành). `index` = thứ tự trong ải.
#[allow(clippy::too_many_arguments)]
pub fn spawn_enemy(
    stage: i64, boss: bool, index: i64, forced: Option<&str>,
    player_x: f64, player_y: f64, view_w: f64, view_h: f64, rolls: SpawnRolls,
) -> EnemyInit {
    let s = stage as f64;
    let angle = rolls.angle_u * TAU + 0.0;
    let dist = view_w.max(view_h) * (rolls.dist_u * (0.95 - 0.62) + 0.62);
    let hidden_boss = forced.is_some();
    let kind: &str = if boss {
        "dinhhanh"
    } else if let Some(f) = forced {
        f
    } else {
        ENEMY_ORDER[((index + stage - 1).rem_euclid(ENEMY_ORDER.len() as i64)) as usize]
    };
    let is_boss = boss || hidden_boss;
    let base = enemy_profile(kind).expect("không có enemyProfile");
    let difficulty = 1.0 + s.powf(0.72);
    let boss_tier = if is_boss { (s / STAGES_PER_DUNGEON as f64).ceil() as i64 } else { 0 };
    let tier = boss_tier as f64;
    let hm = balance::enemy_hp_mult(s);
    let dm = balance::enemy_dmg_mult(s);
    let bh = balance::boss_hp_mult(s, boss);
    let bd = balance::boss_dmg_mult(s, boss);
    let hp_extra = if is_boss { if boss { 1.0 + tier * 0.28 } else { 2.4 } } else { 1.0 };
    let dmg_extra = if is_boss { if boss { 1.0 + tier * 0.06 } else { 1.2 } } else { 1.0 };
    let mut hp = (base.hp * difficulty * hm * bh * hp_extra).floor();
    let mut damage = (base.damage * dm * bd * (0.8 + difficulty * 0.18) * dmg_extra).ceil();
    let mut r = base.r;
    if hidden_boss {
        r *= 1.35;
    }
    let mut max_hp = hp;
    let rate = if hidden_boss {
        (base.rate / 4.0).max(180.0)
    } else if is_boss {
        (base.rate / (1.0 + tier * 0.08)).max(450.0)
    } else {
        base.rate
    };
    let x = (player_x + angle.cos() * dist).clamp(60.0, WW - 60.0);
    let y = (player_y + angle.sin() * dist).clamp(70.0, WH - 70.0);
    let last_attack = rolls.last_attack_u * 1000.0;
    let mut elite = false;
    if !is_boss && rolls.elite_u < (0.09f64).min(0.035 + s * 0.0002) {
        elite = true;
        hp = js_round(hp * 2.4);
        max_hp = hp;
        damage = js_round(damage * 1.6);
        r *= 1.18;
    }
    EnemyInit {
        kind: base.key, name: base.name, color: base.color,
        hp, max_hp, damage, rate, r, speed: base.speed, is_boss, hidden_boss, boss_tier,
        damage_taken_mult: if boss { 0.2 } else if hidden_boss { 0.7 } else { 1.0 },
        damage_out_mult: if boss { 1.0 } else if hidden_boss { 1.2 } else { 1.0 },
        freeze_resist: is_boss,
        reflect: boss && boss_tier >= 3,
        lifesteal: if boss && boss_tier >= 8 { 0.03 } else { 0.0 },
        elite, x, y, last_attack,
        skill_timer: if is_boss { 4.0 } else { f64::INFINITY },
        teleport_timer: if boss { 15.0 } else { f64::INFINITY },
        time_stop_timer: if boss && boss_tier >= 2 { 10.0 } else { f64::INFINITY },
        black_hole_timer: if boss && boss_tier >= 7 { 20.0 } else { f64::INFINITY },
    }
}

/// Tốc độ di chuyển của quái (px/giây): `speed × (boss?2.4:2.8) × (1+min(.25, ải×.002))`, câm lặng ×0,3.
pub fn enemy_move_speed(speed: f64, is_boss: bool, stage: i64, silenced: bool) -> f64 {
    (if silenced { 0.3 } else { 1.0 }) * speed * (if is_boss { 2.4 } else { 2.8 }) * (1.0 + (0.25f64).min(stage as f64 * 0.002))
}

// ---------------- XP & thưởng ----------------
pub const MAX_LEVEL: i64 = 800;

/// `xpNeed`: cấp 1 = 80, từ cấp 2: `floor(80 + cấp×24)`.
pub fn xp_need(level: i64) -> i64 {
    if level <= 1 { 80 } else { (80.0 + level as f64 * 24.0).floor() as i64 }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XpState {
    pub xp: f64,
    pub level: i64,
    pub need: i64,
}
impl XpState {
    pub fn new(level: i64, xp: f64) -> Self {
        XpState { xp, level, need: xp_need(level) }
    }
    /// `gainXP(n)` — trả về số cấp vừa lên.
    pub fn gain(&mut self, n: f64, xp_mult: f64) -> i64 {
        let g = n * xp_mult;
        self.xp += g;
        let mut ups = 0;
        while self.xp >= self.need as f64 && self.level < MAX_LEVEL {
            self.xp -= self.need as f64;
            self.level += 1;
            self.need = (80.0 + self.level as f64 * 24.0).floor() as i64;
            ups += 1;
        }
        if self.level >= MAX_LEVEL {
            self.xp = self.need as f64 - 1.0;
        }
        ups
    }
}

pub fn kill_gold(stage: i64, gold_mult: f64) -> f64 {
    js_round(balance::gold_per_kill(stage as f64) * gold_mult)
}
pub fn kill_gems(is_boss: bool) -> i64 { if is_boss { 10 } else { 1 } }
pub fn kill_xp(is_boss: bool) -> f64 { if is_boss { 120.0 } else { 18.0 } }

// ---------------- chỉ số người chơi (applyClass, không trang bị/kỹ năng) ----------------
#[derive(Clone, Copy, Debug, Default)]
pub struct Upgrades {
    pub power: i64,
    pub speed: i64,
    pub health: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerStats {
    pub max_hp: f64,
    pub speed: f64,
    pub damage: f64,
    pub range: f64,
    pub rate: f64,
}

pub fn player_stats(c: &ClassDef, stage: i64, up: Upgrades) -> PlayerStats {
    let base_hp = js_round(c.hp + (stage - 1) as f64 * 100.0);
    let max_hp = js_round(base_hp * (1.0 + UP_PCT).powf(up.health as f64));
    let speed = c.speed * 1.18 * (1.0 + SPEED_CAP.min(up.speed as f64 * UP_PCT));
    let damage = c.damage * (1.0 + UP_PCT).powf(up.power as f64);
    PlayerStats { max_hp, speed, damage, range: c.range, rate: c.rate }
}
