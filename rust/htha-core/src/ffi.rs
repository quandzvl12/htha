//! Cổng nối WebAssembly: chỉ dùng số (f64 / i32) nên JS gọi trực tiếp, không cần wasm-bindgen.

use crate::{balance, dungeon};

#[no_mangle]
pub extern "C" fn core_version() -> i32 {
    1
}

#[no_mangle]
pub extern "C" fn enemy_hp_mult(stage: f64) -> f64 {
    balance::enemy_hp_mult(stage)
}
#[no_mangle]
pub extern "C" fn enemy_dmg_mult(stage: f64) -> f64 {
    balance::enemy_dmg_mult(stage)
}
#[no_mangle]
pub extern "C" fn boss_hp_mult(stage: f64, is_boss: i32) -> f64 {
    balance::boss_hp_mult(stage, is_boss != 0)
}
#[no_mangle]
pub extern "C" fn boss_dmg_mult(stage: f64, is_boss: i32) -> f64 {
    balance::boss_dmg_mult(stage, is_boss != 0)
}
#[no_mangle]
pub extern "C" fn gold_per_kill(stage: f64) -> f64 {
    balance::gold_per_kill(stage)
}
#[no_mangle]
pub extern "C" fn sell_value(stage: f64, rarity_mult: f64, enhance: f64) -> f64 {
    balance::sell_value(stage, rarity_mult, enhance)
}
#[no_mangle]
pub extern "C" fn upgrade_cost(kind: i32, level: f64) -> f64 {
    balance::upgrade_cost(kind.max(0) as usize, level)
}
#[no_mangle]
pub extern "C" fn skill_value(base: f64, tier_mult: f64) -> f64 {
    balance::skill_value(base, tier_mult)
}
#[no_mangle]
pub extern "C" fn dungeon_of_stage(stage: i32) -> i32 {
    dungeon::dungeon_of_stage(stage)
}
#[no_mangle]
pub extern "C" fn floor_of_stage(stage: i32) -> i32 {
    dungeon::floor_of_stage(stage)
}
#[no_mangle]
pub extern "C" fn rank_index(dungeon: i32) -> i32 {
    dungeon::rank_index(dungeon)
}
#[no_mangle]
pub extern "C" fn region_index(dungeon: i32, regions_len: i32, aspects_len: i32) -> i32 {
    dungeon::region_index(dungeon, regions_len, aspects_len)
}
#[no_mangle]
pub extern "C" fn aspect_index(dungeon: i32, aspects_len: i32) -> i32 {
    dungeon::aspect_index(dungeon, aspects_len)
}
