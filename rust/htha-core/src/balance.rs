//! Công thức cân bằng — tương đương `const BAL`, `upgradeCost`, hệ số kỹ năng trong index.html.

pub const GOLD_PER_KILL: f64 = 30.0;
pub const GOLD_STAGE_K: f64 = 0.005;
pub const SELL_BASE: f64 = 300.0;
pub const SELL_STAGE_K: f64 = 0.02;
pub const SELL_EXP: f64 = 0.9;
pub const SKILL_SCALE: f64 = 0.6;

pub const ENEMY_HP_A: f64 = 0.1244;
pub const ENEMY_HP_B: f64 = 1.2144;
pub const ENEMY_DMG_A: f64 = 0.2093;
pub const ENEMY_DMG_B: f64 = 1.4806;
pub const BOSS_HP_A: f64 = 7.75;
pub const BOSS_HP_B: f64 = -0.811;
pub const BOSS_DMG_A: f64 = 4.199;
pub const BOSS_DMG_B: f64 = -0.364;

/// Giá nâng cấp gốc cho [sức mạnh, tốc độ, máu].
pub const UPGRADE_BASE: [f64; 3] = [20.0, 20.0, 20.0];
pub const UP_COST_PCT: f64 = 0.20;
/// Mỗi cấp nâng cấp tăng +10% chỉ số.
pub const UP_PCT: f64 = 0.10;
pub const SPEED_CAP: f64 = 3.0;

/// `Math.round` của JavaScript (làm tròn nửa lên) — để kết quả khớp từng đơn vị với bản JS.
#[inline]
pub fn js_round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// Nhân máu quái thường: `max(1, A·ải^B)`.
pub fn enemy_hp_mult(stage: f64) -> f64 {
    (ENEMY_HP_A * stage.powf(ENEMY_HP_B)).max(1.0)
}

/// Nhân sát thương quái thường: `max(1, A·ải^B)`.
pub fn enemy_dmg_mult(stage: f64) -> f64 {
    (ENEMY_DMG_A * stage.powf(ENEMY_DMG_B)).max(1.0)
}

/// Nhân máu boss (chỉ áp dụng khi là boss, quái thường = 1).
pub fn boss_hp_mult(stage: f64, is_boss: bool) -> f64 {
    if is_boss {
        (BOSS_HP_A * stage.powf(BOSS_HP_B)).max(0.0001)
    } else {
        1.0
    }
}

/// Nhân sát thương boss (quái thường = 1).
pub fn boss_dmg_mult(stage: f64, is_boss: bool) -> f64 {
    if is_boss {
        (BOSS_DMG_A * stage.powf(BOSS_DMG_B)).max(0.0001)
    } else {
        1.0
    }
}

/// Vàng mỗi quái (chưa làm tròn, chưa nhân hệ số vàng của người chơi): `30 × (1 + 0,5% × ải)`.
pub fn gold_per_kill(stage: f64) -> f64 {
    GOLD_PER_KILL * (1.0 + stage * GOLD_STAGE_K)
}

/// Giá bán trang bị đã làm tròn: `300 × (1 + 2%·ải) × độ_hiếm^0,9 × (1 + 25%·cấp_cường_hóa)`.
/// Thứ tự phép nhân giữ nguyên như JS để không lệch số thực.
pub fn sell_value(stage: f64, rarity_mult: f64, enhance: f64) -> f64 {
    let stage_factor = 1.0 + stage * SELL_STAGE_K;
    let enhance_factor = 1.0 + enhance * 0.25;
    js_round(SELL_BASE * stage_factor * rarity_mult.powf(SELL_EXP) * enhance_factor)
}

/// Giá nâng cấp: `max(1, round(base × 1,2^cấp))`. `kind`: 0 = sức mạnh, 1 = tốc độ, 2 = máu.
pub fn upgrade_cost(kind: usize, level: f64) -> f64 {
    let base = UPGRADE_BASE.get(kind).copied().unwrap_or(UPGRADE_BASE[0]);
    js_round(base * (1.0 + UP_COST_PCT).powf(level)).max(1.0)
}

/// Giá trị kỹ năng khi chọn lúc lên cấp: `max(1, round(b × t × 0,6))`.
pub fn skill_value(base: f64, tier_mult: f64) -> f64 {
    js_round(base * tier_mult * SKILL_SCALE).max(1.0)
}
