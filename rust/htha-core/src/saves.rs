//! Quản lý save theo từng nghề — port logic thuần của `writeClassSave` / `deleteClassSave` / `newest`
//! (dùng chung cho tài khoản `{save, saves}` và khách `{hanhTrinhAnhHungSave, hanhTrinhAnhHungSaves}`).
//! Việc đọc/ghi localStorage vẫn do JS làm; Rust chỉ biến đổi trạng thái.

use crate::jsnum::{to_number, truthy};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct SaveState {
    /// Bản đang chơi (`rec.save` / `hanhTrinhAnhHungSave`), `Value::Null` nếu chưa có.
    pub last: Value,
    /// Các nghề khác theo thứ tự chèn (`rec.saves` / `hanhTrinhAnhHungSaves`).
    pub saves: Map<String, Value>,
}

fn class_key_str(v: &Value) -> Option<&str> {
    match v.get("classKey") {
        Some(Value::String(s)) if !s.is_empty() => Some(s),
        _ => None,
    }
}

/// Bản save mới nhất theo `savedAt` (hòa -> bản đứng trước trong map), như `newest()` trong JS.
fn newest(saves: &Map<String, Value>) -> Option<Value> {
    let mut best: Option<(&Value, f64)> = None;
    for v in saves.values() {
        if !truthy(Some(v)) || !truthy(v.get("classKey")) {
            continue;
        }
        let t = to_number(v.get("savedAt"));
        let t = if t.is_nan() { 0.0 } else { t };
        if best.map(|(_, bt)| t > bt).unwrap_or(true) {
            best = Some((v, t));
        }
    }
    best.map(|(v, _)| v.clone())
}

pub fn write_class_save(mut st: SaveState, data: Value) -> SaveState {
    if let Some(prev_key) = class_key_str(&st.last).map(str::to_string) {
        if st.last.get("classKey") != data.get("classKey") {
            st.saves.insert(prev_key, st.last.clone());
        }
    }
    if let Some(Value::String(k)) = data.get("classKey") {
        if truthy(st.saves.get(k)) {
            st.saves.shift_remove(k);
        }
    }
    st.last = data;
    st
}

pub fn delete_class_save(mut st: SaveState, key: &str) -> SaveState {
    st.saves.shift_remove(key);
    if class_key_str(&st.last) == Some(key) {
        match newest(&st.saves) {
            Some(n) => {
                if let Some(nk) = class_key_str(&n).map(str::to_string) {
                    st.saves.shift_remove(&nk);
                }
                st.last = n;
            }
            None => st.last = Value::Null,
        }
    }
    st
}

pub fn state_from_json(v: &Value) -> SaveState {
    SaveState {
        last: v.get("last").cloned().unwrap_or(Value::Null),
        saves: v.get("saves").and_then(Value::as_object).cloned().unwrap_or_default(),
    }
}

pub fn state_to_json(st: SaveState) -> Value {
    let mut m = Map::new();
    m.insert("last".into(), st.last);
    m.insert("saves".into(), Value::Object(st.saves));
    Value::Object(m)
}
