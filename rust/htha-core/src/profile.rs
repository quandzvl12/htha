//! Hồ sơ người chơi — port nguyên văn `profileXPForNextLevel`, `profileLevelInfo`,
//! `cleanProfileName`, `normalizePlayerProfile` từ index.html.

use crate::data_gen::PROFILE_AVATARS;
use crate::dungeon::{MAX_DUNGEONS, STAGES_PER_DUNGEON};
use crate::jsnum::*;
use serde_json::{json, Value};

pub const MAX_STAGES: i64 = (MAX_DUNGEONS * STAGES_PER_DUNGEON) as i64;
const DEFAULT_NAME: &str = "Người chơi";

pub fn xp_for_next_level(level: i64) -> i64 {
    120 + level * 30
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LevelInfo {
    pub level: i64,
    pub progress: i64,
    pub need: i64,
}

pub fn level_info(total_xp: f64) -> LevelInfo {
    let n = if total_xp.is_nan() || total_xp == 0.0 { 0.0 } else { total_xp };
    let mut remaining = floor_i64(n).max(0);
    let mut level = 1i64;
    let mut need = xp_for_next_level(level);
    while level < 100 && remaining >= need {
        remaining -= need;
        level += 1;
        need = xp_for_next_level(level);
    }
    LevelInfo {
        level,
        progress: if level >= 100 { 0 } else { remaining },
        need: if level >= 100 { 0 } else { need },
    }
}

pub fn clean_name(value: Option<&Value>, fallback: &str) -> String {
    let raw = if truthy(value) { js_string(value) } else { String::new() };
    let no_ctrl: String = raw
        .chars()
        .filter(|c| !matches!(*c, '\u{0}'..='\u{1f}' | '\u{7f}' | '<' | '>'))
        .collect();
    let mut collapsed = String::new();
    let mut in_space = false;
    for c in no_ctrl.chars() {
        if is_js_space(c) {
            if !in_space {
                collapsed.push(' ');
            }
            in_space = true;
        } else {
            collapsed.push(c);
            in_space = false;
        }
    }
    let name = slice_utf16(js_trim(&collapsed), 20);
    if !name.is_empty() {
        return name;
    }
    let fb = if fallback.is_empty() { DEFAULT_NAME } else { fallback };
    let fb = slice_utf16(fb, 20);
    if fb.is_empty() { DEFAULT_NAME.to_string() } else { fb }
}

fn clamp_floor(v: Option<&Value>, fallback: f64, lo: i64, hi: i64) -> i64 {
    floor_i64(num_or(v, fallback)).min(hi).max(lo)
}

fn normalize_history_item(item: &Value, class_keys: &[String], now_ms: f64) -> Value {
    let empty = serde_json::Map::new();
    let o = item.as_object().unwrap_or(&empty);
    let stage = clamp_floor(o.get("stage"), 1.0, 1, MAX_STAGES);
    let dungeon_default = (stage as f64 / STAGES_PER_DUNGEON as f64).ceil();
    let class_key = match o.get("classKey") {
        Some(Value::String(k)) if class_keys.iter().any(|c| c == k) => k.clone(),
        _ => "warrior".to_string(),
    };
    let reason = if truthy(o.get("reason")) { js_string(o.get("reason")) } else { "Kết thúc trận".into() };
    let mode = if truthy(o.get("mode")) { js_string(o.get("mode")) } else { "Dungeon".into() };
    json!({
        "at": floor_i64(num_or(o.get("at"), now_ms)).max(1),
        "classKey": class_key,
        "stage": stage,
        "dungeon": clamp_floor(o.get("dungeon"), dungeon_default, 1, MAX_DUNGEONS as i64),
        "level": floor_i64(num_or(o.get("level"), 1.0)).max(1),
        "kills": floor_i64(num_or(o.get("kills"), 0.0)).max(0),
        "duration": floor_i64(num_or(o.get("duration"), 0.0)).max(0),
        "xpGained": floor_i64(num_or(o.get("xpGained"), 0.0)).max(0),
        "reason": slice_utf16(&reason, 40),
        "mode": slice_utf16(&mode, 24),
    })
}

/// `normalizePlayerProfile(raw, fallbackName)`. `class_keys` = `Object.keys(classes)`, `now_ms` = `Date.now()`.
pub fn normalize_profile(raw: &Value, fallback_name: &str, class_keys: &[String], now_ms: f64) -> Value {
    let empty = serde_json::Map::new();
    let o = raw.as_object().unwrap_or(&empty);
    let total_xp = floor_i64(num_or(o.get("totalXp"), 0.0)).max(0);
    let info = level_info(total_xp as f64);
    let history: Vec<Value> = match o.get("history") {
        Some(Value::Array(a)) => a.iter().take(20).map(|it| normalize_history_item(it, class_keys, now_ms)).collect(),
        _ => Vec::new(),
    };
    let avatar = match o.get("avatar") {
        Some(Value::String(a)) if PROFILE_AVATARS.contains(&a.as_str()) => a.clone(),
        _ => PROFILE_AVATARS[0].to_string(),
    };
    let runs = to_number(o.get("totalRuns"));
    let total_runs = floor_i64(if runs.is_finite() { runs } else { history.len() as f64 }).max(0);
    json!({
        "name": clean_name(o.get("name"), fallback_name),
        "avatar": avatar,
        "totalXp": total_xp,
        "level": info.level,
        "levelXp": info.progress,
        "totalRuns": total_runs,
        "bestStage": clamp_floor(o.get("bestStage"), 1.0, 1, MAX_STAGES),
        "totalKills": floor_i64(num_or(o.get("totalKills"), 0.0)).max(0),
        "playSeconds": floor_i64(num_or(o.get("playSeconds"), 0.0)).max(0),
        "history": history,
    })
}
