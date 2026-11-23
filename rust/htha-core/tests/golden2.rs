//! Bước 2 — đối chiếu Rust với CHÍNH các hàm JS gốc (dữ liệu sinh bởi tools/gen_golden2.js).
use htha_core::{ffi, profile, saves};
use serde_json::Value;

fn load(name: &str) -> Value {
    let p = format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), name);
    serde_json::from_str(&std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{p}: {e}"))).unwrap()
}

#[test]
fn profile_level_info_matches_js() {
    let g = load("golden_profile.json");
    for c in g["levels"].as_array().unwrap() {
        let xp = c[0].as_f64().unwrap();
        let e = &c[1];
        let r = profile::level_info(xp);
        assert_eq!(r.level, e["level"].as_i64().unwrap(), "level xp={xp}");
        assert_eq!(r.progress, e["progress"].as_i64().unwrap(), "progress xp={xp}");
        assert_eq!(r.need, e["need"].as_i64().unwrap(), "need xp={xp}");
    }
}

#[test]
fn normalize_profile_matches_js_on_612_cases() {
    let g = load("golden_profile.json");
    let now = g["now"].as_f64().unwrap();
    let classes: Vec<String> = g["classKeys"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    for (i, c) in g["cases"].as_array().unwrap().iter().enumerate() {
        let got = profile::normalize_profile(&c["raw"], c["fallback"].as_str().unwrap(), &classes, now);
        assert_eq!(got, c["expected"], "ca #{i}\n raw = {}\n fallback = {}", c["raw"], c["fallback"]);
    }
}

fn state_of(v: &Value) -> saves::SaveState {
    saves::state_from_json(v)
}

#[test]
fn class_saves_match_js_for_guest_and_account() {
    let g = load("golden_saves.json");
    let mut total = 0;
    for (n, case) in g.as_array().unwrap().iter().enumerate() {
        let mut st = saves::SaveState { last: Value::Null, saves: Default::default() };
        for (k, op) in case["ops"].as_array().unwrap().iter().enumerate() {
            st = match op["op"].as_str().unwrap() {
                "write" => saves::write_class_save(st, op["data"].clone()),
                "delete" => saves::delete_class_save(st, op["key"].as_str().unwrap()),
                o => panic!("op lạ {o}"),
            };
            let want = state_of(&op["after"]);
            assert_eq!(st, want, "chuỗi #{n} ({}) thao tác #{k}: {}", case["mode"], op);
            total += 1;
        }
    }
    assert!(total > 1500);
}

fn call(f: impl FnOnce() -> i32) -> (i32, String) {
    let code = f();
    let len = ffi::htha_out_len() as usize;
    let s = unsafe { std::slice::from_raw_parts(ffi::htha_out_ptr(), len) };
    (code, String::from_utf8(s.to_vec()).unwrap())
}

#[test]
fn ffi_string_layer_roundtrip() {
    let raw = r#"{"name":"  Quân  ","totalXp":500,"history":[{"stage":"12"}]}"#;
    let classes = r#"["warrior","mage"]"#;
    let fb = "khách";
    let (code, out) = call(|| unsafe {
        ffi::normalize_profile(raw.as_ptr(), raw.len() as u32, fb.as_ptr(), fb.len() as u32, classes.as_ptr(), classes.len() as u32, 1.7e12)
    });
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["name"], "Quân");
    assert_eq!(v["level"], 3);
    assert_eq!(v["levelXp"], 170);
    assert_eq!(v["history"][0]["stage"], 12);
    assert_eq!(v["history"][0]["classKey"], "warrior");

    let bad = "{không phải json";
    let (code, msg) = call(|| unsafe {
        ffi::normalize_profile(bad.as_ptr(), bad.len() as u32, fb.as_ptr(), fb.len() as u32, classes.as_ptr(), classes.len() as u32, 0.0)
    });
    assert_eq!(code, 1);
    assert!(msg.contains("JSON"));

    let st = r#"{"last":{"classKey":"mage","savedAt":5},"saves":{}}"#;
    let data = r#"{"classKey":"archer","savedAt":9}"#;
    let (code, out) = call(|| unsafe { ffi::class_save_write(st.as_ptr(), st.len() as u32, data.as_ptr(), data.len() as u32) });
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["last"]["classKey"], "archer");
    assert_eq!(v["saves"]["mage"]["savedAt"], 5);

    let key = "archer";
    let (code, out) = call(|| unsafe { ffi::class_save_delete(out.as_ptr(), out.len() as u32, key.as_ptr(), key.len() as u32) });
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["last"]["classKey"], "mage", "xóa bản đang chơi -> lấy bản mới nhất còn lại");
    assert!(v["saves"].as_object().unwrap().is_empty());
}

#[test]
fn alloc_dealloc_and_data_json() {
    let p = ffi::htha_alloc(16);
    assert!(!p.is_null());
    unsafe { ffi::htha_dealloc(p, 16) };
    assert_eq!(ffi::data_json(), 0);
    let len = ffi::htha_out_len() as usize;
    let s = unsafe { std::slice::from_raw_parts(ffi::htha_out_ptr(), len) };
    let v: Value = serde_json::from_slice(s).unwrap();
    assert_eq!(v["rarities"].as_array().unwrap().len(), 5);
    assert_eq!(v["enemyProfiles"]["dinhhanh"]["isBoss"], true);
    assert_eq!(v["enemyProfiles"]["duc"]["hp"], 38.0);
}
