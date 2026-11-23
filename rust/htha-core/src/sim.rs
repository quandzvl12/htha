//! BƯỚC 4a — mô phỏng chiến đấu thuần Rust (không đồ họa), dùng chung cho bản desktop/WASM/Android.
//!
//! PHẠM VI (lát cắt có chủ đích): một nghề, quái thường + boss Dinh Hạnh ở các ải ×10, đạn/va chạm/cận chiến,
//! XP/vàng/ngọc, chuyển ải. CHƯA có: kỹ năng, trang bị, thú cưng, triệu hồi, chiêu boss, chợ, hạt/hiệu ứng.
//! Quy tắc di chuyển/va chạm/bắn được port từ vòng lặp `update()` của index.html; riêng phần SINH QUÁI,
//! chỉ số người chơi, XP và thưởng đã được đối chiếu với hàm JS gốc (xem tests/golden_combat.rs).

use crate::combat::*;
use crate::data_gen::ClassDef;

/// Bộ sinh số giả ngẫu nhiên xác định (xorshift64*), để chạy lại được từ một hạt giống.
#[derive(Clone, Debug)]
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self { Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1) }
    pub fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 >> 12; self.0 ^= self.0 << 25; self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn range(&mut self, a: f64, b: f64) -> f64 { self.next_f64() * (b - a) + a }
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub e: EnemyInit,
    pub dead: bool,
    /// Dùng chung cho nháy khi bị đánh VÀ hồi chiêu va chạm của quái — giống hệt `e.hit` trong JS.
    pub hit: f64,
}
#[derive(Clone, Debug)]
pub struct Shot { pub x: f64, pub y: f64, pub vx: f64, pub vy: f64, pub r: f64, pub damage: f64, pub life: f64, pub pierce: i32, pub hit_ids: Vec<usize> }
#[derive(Clone, Debug)]
pub struct EnemyShot { pub x: f64, pub y: f64, pub vx: f64, pub vy: f64, pub r: f64, pub damage: f64, pub life: f64, pub boss: bool }
#[derive(Clone, Debug)]
pub struct Player {
    pub x: f64, pub y: f64, pub r: f64, pub hp: f64, pub max_hp: f64,
    pub speed: f64, pub damage: f64, pub range: f64, pub rate: f64,
    pub face_x: f64, pub face_y: f64, pub flash: f64,
}

/// Sự kiện để giao diện hiển thị (âm thanh, chữ nổi, hạt) mà không cần đọc lại trạng thái.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Shoot, MeleeHit { x: f64, y: f64 }, EnemyHit { x: f64, y: f64 },
    EnemyKilled { x: f64, y: f64, boss: bool, gold: f64 },
    PlayerHurt { amount: f64 }, LevelUp { level: i64 }, StageStart { stage: i64 }, GameOver,
}

pub struct Sim {
    pub class: &'static ClassDef,
    pub melee: bool,
    pub stage: i64,
    pub up: Upgrades,
    pub xp: XpState,
    pub kills: i64,
    pub coins: f64,
    pub gems: i64,
    pub time: f64,
    pub last_shot: f64,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub shots: Vec<Shot>,
    pub eshots: Vec<EnemyShot>,
    pub game_over: bool,
    pub events: Vec<Event>,
    pub view_w: f64,
    pub view_h: f64,
    rng: Rng,
}

impl Sim {
    pub fn new(class_key: &str, seed: u64, view_w: f64, view_h: f64) -> Option<Sim> {
        let class = class_def(class_key)?;
        if class.god { return None; }
        let up = Upgrades::default();
        let st = player_stats(class, 1, up);
        let mut s = Sim {
            class, melee: class.melee || class.key == "warrior", stage: 1, up, xp: XpState::new(1, 0.0),
            kills: 0, coins: 0.0, gems: 0, time: 0.0, last_shot: -999.0,
            player: Player { x: WW / 2.0, y: WH / 2.0, r: PLAYER_R, hp: st.max_hp, max_hp: st.max_hp, speed: st.speed, damage: st.damage,
                range: st.range, rate: st.rate, face_x: 0.0, face_y: 1.0, flash: 0.0 },
            enemies: vec![], shots: vec![], eshots: vec![], game_over: false, events: vec![], view_w, view_h, rng: Rng::new(seed),
        };
        s.start_stage();
        Some(s)
    }

    /// `startStage()`: sinh `min(26, 8+⌊ải×0,65⌋)` quái; ải ×10 con cuối là boss; 12% có boss ẩn.
    pub fn start_stage(&mut self) {
        self.enemies.clear(); self.shots.clear(); self.eshots.clear();
        self.time = 0.0; self.last_shot = -999.0;
        self.player.hp = self.player.max_hp.min(self.player.hp + self.player.max_hp * 0.25);
        let count = stage_enemy_count(self.stage);
        let boss_stage = is_dungeon_boss_stage(self.stage);
        for i in 0..count { self.spawn(boss_stage && i == count - 1, i, None); }
        if !boss_stage && self.rng.next_f64() < 0.12 {
            let forced = if self.rng.next_f64() < 0.5 { "tayhung" } else { "thanh" };
            self.spawn(false, count, Some(forced));
        }
        self.events.push(Event::StageStart { stage: self.stage });
    }

    fn spawn(&mut self, boss: bool, index: i64, forced: Option<&str>) {
        let rolls = SpawnRolls { angle_u: self.rng.next_f64(), dist_u: self.rng.next_f64(), last_attack_u: self.rng.next_f64(), elite_u: self.rng.next_f64() };
        let init = spawn_enemy(self.stage, boss, index, forced, self.player.x, self.player.y, self.view_w, self.view_h, rolls);
        if init.is_boss && boss { self.player.hp = self.player.hp.min(self.player.max_hp * 0.5); }
        else if init.is_boss { self.player.hp = self.player.hp.min(self.player.max_hp * 0.75); }
        self.enemies.push(Enemy { e: init, dead: false, hit: 0.0 });
    }

    fn nearest(&self) -> Option<usize> {
        let (px, py) = (self.player.x, self.player.y);
        self.enemies.iter().enumerate().filter(|(_, e)| !e.dead)
            .min_by(|(_, a), (_, b)| {
                let da = (a.e.x - px).powi(2) + (a.e.y - py).powi(2);
                let db = (b.e.x - px).powi(2) + (b.e.y - py).powi(2);
                da.partial_cmp(&db).unwrap()
            }).map(|(i, _)| i)
    }

    fn hurt(&mut self, n: f64) {
        if self.game_over { return; }
        self.player.hp -= n;
        self.player.flash = 0.18;
        self.events.push(Event::PlayerHurt { amount: n });
        if self.player.hp <= 0.0 {
            self.player.hp = 0.0;
            self.game_over = true;
            self.events.push(Event::GameOver);
        }
    }

    fn kill(&mut self, i: usize) {
        if self.enemies[i].dead { return; }
        self.enemies[i].dead = true;
        let (x, y, boss) = { let e = &self.enemies[i].e; (e.x, e.y, e.is_boss) };
        self.kills += 1;
        let g = kill_gold(self.stage, 1.0);
        self.coins += g;
        self.gems += kill_gems(boss);
        let ups = self.xp.gain(kill_xp(boss), 1.0);
        self.events.push(Event::EnemyKilled { x, y, boss, gold: g });
        for _ in 0..ups { self.events.push(Event::LevelUp { level: self.xp.level }); }
    }

    /// Một bước mô phỏng. `input` là vectơ hướng (−1..1).
    pub fn step(&mut self, dt: f64, input: (f64, f64)) {
        if self.game_over { return; }
        self.time += dt;
        self.player.flash = (self.player.flash - dt).max(0.0);

        // Di chuyển người chơi (kẹp biên 25 / 40 như JS)
        if input.0.hypot(input.1) > 0.1 { self.player.face_x = input.0; self.player.face_y = input.1; }
        self.player.x = (self.player.x + input.0 * self.player.speed * dt).clamp(25.0, WW - 25.0);
        self.player.y = (self.player.y + input.1 * self.player.speed * dt).clamp(40.0, WH - 40.0);

        // Tự động tấn công mục tiêu gần nhất
        if let Some(t) = self.nearest() {
            if self.time - self.last_shot >= self.player.rate / 1000.0 {
                let (tx, ty, tr) = { let e = &self.enemies[t].e; (e.x, e.y, e.r) };
                if self.melee {
                    self.player.face_x = tx - self.player.x; self.player.face_y = ty - self.player.y;
                    let reach = self.player.r + tr + 42.0;
                    if (tx - self.player.x).hypot(ty - self.player.y) <= reach {
                        let dmg = self.player.damage * self.enemies[t].e.damage_taken_mult;
                        self.enemies[t].e.hp -= dmg;
                        self.enemies[t].hit = 0.16;
                        self.events.push(Event::MeleeHit { x: tx, y: ty });
                        if self.enemies[t].e.hp <= 0.0 { self.kill(t); }
                        self.last_shot = self.time;
                    }
                } else {
                    let a = (ty - self.player.y).atan2(tx - self.player.x);
                    self.shots.push(Shot { x: self.player.x, y: self.player.y, vx: a.cos() * 390.0, vy: a.sin() * 390.0,
                        r: if self.class.key == "archer" { 4.0 } else { 5.0 }, damage: self.player.damage, life: 1.8, pierce: 1, hit_ids: vec![] });
                    self.last_shot = self.time;
                    self.events.push(Event::Shoot);
                }
            }
        }

        // Quái: di chuyển / va chạm / bắn
        let (px, py, pr) = (self.player.x, self.player.y, self.player.r);
        let stage = self.stage;
        let mut contact: Vec<f64> = vec![];
        let mut new_shots: Vec<EnemyShot> = vec![];
        for en in self.enemies.iter_mut().filter(|e| !e.dead) {
            en.hit = (en.hit - dt).max(0.0);
            let (dx, dy) = (px - en.e.x, py - en.e.y);
            let d = (dx * dx + dy * dy).sqrt();
            let d = if d == 0.0 { 1.0 } else { d };
            if d > en.e.r + pr + 4.0 {
                let sp = enemy_move_speed(en.e.speed, en.e.is_boss, stage, false);
                en.e.x += dx / d * sp * dt;
                en.e.y += dy / d * sp * dt;
            } else if en.hit <= 0.0 {
                contact.push(en.e.damage * en.e.damage_out_mult);
                en.hit = en.e.rate / 1000.0;
            }
            en.e.last_attack -= dt * 1000.0;
            if en.e.last_attack <= 0.0 && d < 380.0 {
                en.e.last_attack = en.e.rate;
                en.e.x -= dx / d * 8.0;
                new_shots.push(EnemyShot { x: en.e.x, y: en.e.y, vx: dx / d * 120.0, vy: dy / d * 120.0,
                    r: if en.e.is_boss { 8.0 } else { 5.0 }, damage: en.e.damage * 0.7 * en.e.damage_out_mult, life: 3.0, boss: en.e.is_boss });
            }
        }
        for n in contact { self.hurt(n); }
        self.eshots.extend(new_shots);

        // Đạn của người chơi
        let n_shots = self.shots.len();
        for si in 0..n_shots {
            { let s = &mut self.shots[si]; s.x += s.vx * dt; s.y += s.vy * dt; s.life -= dt; }
            for ei in 0..self.enemies.len() {
                let s = &self.shots[si];
                let en = &self.enemies[ei];
                if en.dead || s.life <= 0.0 || s.hit_ids.contains(&ei) { continue; }
                let (dx, dy, r) = (s.x - en.e.x, s.y - en.e.y, s.r + en.e.r);
                if dx.abs() > r || dy.abs() > r || dx * dx + dy * dy >= r * r { continue; }
                let dmg = s.damage * en.e.damage_taken_mult;
                let (ex, ey) = (en.e.x, en.e.y);
                self.enemies[ei].e.hp -= dmg;
                self.enemies[ei].hit = 0.08;
                self.events.push(Event::EnemyHit { x: ex, y: ey });
                if self.enemies[ei].e.hp <= 0.0 { self.kill(ei); }
                let s = &mut self.shots[si];
                s.hit_ids.push(ei); s.pierce -= 1;
                if s.pierce <= 0 { s.life = 0.0; }
            }
        }
        self.shots.retain(|s| s.life > 0.0);

        // Đạn của quái
        let mut hits = vec![];
        for s in self.eshots.iter_mut() {
            s.x += s.vx * dt; s.y += s.vy * dt; s.life -= dt;
            let (dx, dy, r) = (s.x - px, s.y - py, s.r + pr);
            if dx.abs() <= r && dy.abs() <= r && dx * dx + dy * dy < r * r { hits.push(s.damage); s.life = 0.0; }
        }
        for n in hits { self.hurt(n); }
        self.eshots.retain(|s| s.life > 0.0);

        // Hết quái → ải kế: `stage++; maxHp += 100; hp = min(maxHp, hp+100); startStage()` (như advanceFromMarket)
        if !self.game_over && self.enemies.iter().all(|e| e.dead) {
            self.stage += 1;
            self.player.max_hp += 100.0;
            self.player.hp = self.player.max_hp.min(self.player.hp + 100.0);
            self.start_stage();
        }
    }

    /// Lấy và xóa các sự kiện từ lần gọi trước.
    pub fn take_events(&mut self) -> Vec<Event> { std::mem::take(&mut self.events) }
    pub fn alive(&self) -> usize { self.enemies.iter().filter(|e| !e.dead).count() }
}
