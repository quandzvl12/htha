//! Cổng nối WebAssembly: chỉ dùng số (f64 / i32) nên JS gọi trực tiếp, không cần wasm-bindgen.

use crate::{balance, data_gen, dungeon, profile, saves};
use serde_json::Value;
use std::cell::RefCell;

#[no_mangle]
pub extern "C" fn core_version() -> i32 {
    2
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

// ---------------------------------------------------------------------------------------------
// BƯỚC 2 — cổng chuỗi/JSON. JS cấp phát bộ nhớ bằng htha_alloc, ghi UTF-8 vào, gọi hàm,
// đọc kết quả ở htha_out_ptr()/htha_out_len() rồi giải phóng bằng htha_dealloc.
// Hàm trả 0 = thành công (kết quả JSON ở bộ đệm ra), khác 0 = lỗi (bộ đệm ra chứa thông báo).
// ---------------------------------------------------------------------------------------------

thread_local! {
    static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn set_out(s: String) {
    OUT.with(|o| *o.borrow_mut() = s.into_bytes());
}

#[no_mangle]
pub extern "C" fn htha_alloc(len: u32) -> *mut u8 {
    Box::into_raw(vec![0u8; len as usize].into_boxed_slice()) as *mut u8
}

/// # Safety
/// `ptr`/`len` phải đúng cặp đã nhận từ `htha_alloc`.
#[no_mangle]
pub unsafe extern "C" fn htha_dealloc(ptr: *mut u8, len: u32) {
    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len as usize)));
}

#[no_mangle]
pub extern "C" fn htha_out_ptr() -> *const u8 {
    OUT.with(|o| o.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn htha_out_len() -> u32 {
    OUT.with(|o| o.borrow().len() as u32)
}

unsafe fn read<'a>(ptr: *const u8, len: u32) -> Result<&'a str, String> {
    std::str::from_utf8(std::slice::from_raw_parts(ptr, len as usize)).map_err(|e| format!("UTF-8 sai: {e}"))
}

fn finish(r: Result<Value, String>) -> i32 {
    match r {
        Ok(v) => {
            set_out(v.to_string());
            0
        }
        Err(e) => {
            set_out(e);
            1
        }
    }
}

fn parse(s: &str, what: &str) -> Result<Value, String> {
    serde_json::from_str(s).map_err(|e| format!("{what} không phải JSON hợp lệ: {e}"))
}

/// `normalizePlayerProfile(raw, fallbackName)` — `classes_json` = mảng tên nghề, `now_ms` = Date.now().
///
/// # Safety
/// Các con trỏ phải trỏ tới vùng UTF-8 hợp lệ do `htha_alloc` cấp.
#[no_mangle]
pub unsafe extern "C" fn normalize_profile(
    raw_ptr: *const u8, raw_len: u32,
    fb_ptr: *const u8, fb_len: u32,
    classes_ptr: *const u8, classes_len: u32,
    now_ms: f64,
) -> i32 {
    finish((|| {
        let raw = parse(read(raw_ptr, raw_len)?, "hồ sơ")?;
        let fb = read(fb_ptr, fb_len)?;
        let classes: Vec<String> = serde_json::from_str(read(classes_ptr, classes_len)?)
            .map_err(|e| format!("danh sách nghề sai: {e}"))?;
        Ok(profile::normalize_profile(&raw, fb, &classes, now_ms))
    })())
}

/// Ghi một save nghề: trạng thái `{last, saves}` + dữ liệu save mới -> trạng thái mới.
///
/// # Safety
/// Như `normalize_profile`.
#[no_mangle]
pub unsafe extern "C" fn class_save_write(st_ptr: *const u8, st_len: u32, data_ptr: *const u8, data_len: u32) -> i32 {
    finish((|| {
        let st = saves::state_from_json(&parse(read(st_ptr, st_len)?, "trạng thái save")?);
        let data = parse(read(data_ptr, data_len)?, "dữ liệu save")?;
        Ok(saves::state_to_json(saves::write_class_save(st, data)))
    })())
}

/// Xóa save của một nghề.
///
/// # Safety
/// Như `normalize_profile`.
#[no_mangle]
pub unsafe extern "C" fn class_save_delete(st_ptr: *const u8, st_len: u32, key_ptr: *const u8, key_len: u32) -> i32 {
    finish((|| {
        let st = saves::state_from_json(&parse(read(st_ptr, st_len)?, "trạng thái save")?);
        Ok(saves::state_to_json(saves::delete_class_save(st, read(key_ptr, key_len)?)))
    })())
}

#[no_mangle]
pub extern "C" fn profile_level(total_xp: f64) -> i32 {
    profile::level_info(total_xp).level as i32
}
#[no_mangle]
pub extern "C" fn profile_progress(total_xp: f64) -> f64 {
    profile::level_info(total_xp).progress as f64
}
#[no_mangle]
pub extern "C" fn profile_need(total_xp: f64) -> f64 {
    profile::level_info(total_xp).need as f64
}
#[no_mangle]
pub extern "C" fn profile_xp_for_next_level(level: f64) -> f64 {
    profile::xp_for_next_level(jsnum_floor(level)) as f64
}
fn jsnum_floor(n: f64) -> i64 {
    crate::jsnum::floor_i64(n)
}

/// Các bảng dữ liệu (độ hiếm, quái) dạng JSON — để các bước sau dùng chung một nguồn.
#[no_mangle]
pub extern "C" fn data_json() -> i32 {
    let rarities: Vec<Value> = data_gen::RARITIES
        .iter()
        .map(|r| serde_json::json!({"name": r.name, "color": r.color, "mult": r.mult}))
        .collect();
    let mut enemies = serde_json::Map::new();
    for e in data_gen::ENEMY_PROFILES {
        let mut o = serde_json::json!({"name": e.name, "r": e.r, "hp": e.hp, "speed": e.speed,
            "damage": e.damage, "rate": e.rate, "color": e.color});
        if e.is_boss {
            o["isBoss"] = Value::Bool(true);
        }
        enemies.insert(e.key.to_string(), o);
    }
    finish(Ok(serde_json::json!({"rarities": rarities, "enemyProfiles": enemies,
        "profileAvatars": data_gen::PROFILE_AVATARS})))
}
