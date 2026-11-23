//! Bước 4a — đối chiếu lõi chiến đấu Rust với CHÍNH `spawnEnemy` / `applyClass` / `gainXP` trong game
//! (dữ liệu sinh bởi tools/gen_golden3.js, chạy hàm gốc trong jsdom với dòng Math.random cố định).
use htha_core::combat::*;
use serde_json::Value;

fn load() -> Value {
    let p = format!("{}/tests/golden_combat.json", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}
fn f(v: &Value) -> f64 { v.as_f64().unwrap_or(f64::INFINITY) } // null = Infinity

fn close(a: f64, e: f64) -> bool {
    (a.is_infinite() && e.is_infinite()) || (a - e).abs() <= 1e-9 * e.abs().max(1.0)
}

#[test]
fn spawn_enemy_matches_js_on_418_cases() {
    let g = load();
    for (i, c) in g["spawn"].as_array().unwrap().iter().enumerate() {
        let r = c["rolls"].as_array().unwrap();
        let rolls = SpawnRolls { angle_u: f(&r[0]), dist_u: f(&r[1]), last_attack_u: f(&r[2]), elite_u: f(&r[3]) };
        let forced = c["forced"].as_str();
        let e = spawn_enemy(
            c["stage"].as_i64().unwrap(), c["boss"].as_bool().unwrap(), c["index"].as_i64().unwrap(), forced,
            f(&c["px"]), f(&c["py"]), f(&c["W"]), f(&c["H"]), rolls,
        );
        let x = &c["exp"];
        let ctx = format!("ca #{i}: ải {} boss={} index={} forced={:?} elite_u={}", c["stage"], c["boss"], c["index"], forced, rolls.elite_u);
        assert_eq!(e.kind, x["kind"].as_str().unwrap(), "{ctx} kind");
        for (name, got, want) in [
            ("hp", e.hp, f(&x["hp"])), ("maxHp", e.max_hp, f(&x["maxHp"])), ("damage", e.damage, f(&x["damage"])),
            ("rate", e.rate, f(&x["rate"])), ("r", e.r, f(&x["r"])), ("speed", e.speed, f(&x["speed"])),
            ("damageTakenMult", e.damage_taken_mult, f(&x["damageTakenMult"])), ("damageOutMult", e.damage_out_mult, f(&x["damageOutMult"])),
            ("lifesteal", e.lifesteal, f(&x["lifesteal"])), ("x", e.x, f(&x["x"])), ("y", e.y, f(&x["y"])),
            ("lastAttack", e.last_attack, f(&x["lastAttack"])), ("skillTimer", e.skill_timer, f(&x["skillTimer"])),
            ("teleportTimer", e.teleport_timer, f(&x["teleportTimer"])), ("timeStopTimer", e.time_stop_timer, f(&x["timeStopTimer"])),
            ("blackHoleTimer", e.black_hole_timer, f(&x["blackHoleTimer"])),
        ] {
            assert!(close(got, want), "{ctx}: {name} rust={got} js={want}");
        }
        assert_eq!(e.is_boss, x["isBoss"].as_bool().unwrap(), "{ctx} isBoss");
        assert_eq!(e.hidden_boss, x["hiddenBoss"].as_bool().unwrap(), "{ctx} hiddenBoss");
        assert_eq!(e.boss_tier, x["bossTier"].as_i64().unwrap(), "{ctx} bossTier");
        assert_eq!(e.freeze_resist, x["freezeResist"].as_bool().unwrap(), "{ctx} freezeResist");
        assert_eq!(e.reflect, x["reflect"].as_bool().unwrap(), "{ctx} reflect");
        assert_eq!(e.elite, x["elite"].as_bool().unwrap(), "{ctx} elite");
    }
}

#[test]
fn player_stats_match_js_apply_class() {
    let g = load();
    for c in g["player"].as_array().unwrap() {
        let key = c["cls"].as_str().unwrap();
        let up = c["up"].as_array().unwrap();
        let s = player_stats(class_def(key).unwrap(), c["stage"].as_i64().unwrap(),
            Upgrades { power: up[0].as_i64().unwrap(), speed: up[1].as_i64().unwrap(), health: up[2].as_i64().unwrap() });
        let x = &c["exp"];
        let ctx = format!("{key} ải {} nâng cấp {:?}", c["stage"], up);
        assert!(close(s.max_hp, f(&x["maxHp"])), "{ctx}: maxHp rust={} js={}", s.max_hp, x["maxHp"]);
        assert!(close(s.speed, f(&x["speed"])), "{ctx}: speed rust={} js={}", s.speed, x["speed"]);
        assert!(close(s.damage, f(&x["damage"])), "{ctx}: damage rust={} js={}", s.damage, x["damage"]);
        assert!(close(s.range, f(&x["range"])) && close(s.rate, f(&x["rate"])), "{ctx}: range/rate");
    }
}

#[test]
fn xp_matches_js_gain_xp() {
    let g = load();
    for c in g["xp"].as_array().unwrap() {
        let mut s = XpState::new(c["level"].as_i64().unwrap(), f(&c["xp"]));
        s.gain(f(&c["gain"]), 1.0);
        let x = &c["exp"];
        let ctx = format!("cấp {} xp {} +{}", c["level"], c["xp"], c["gain"]);
        assert_eq!(s.level, x["level"].as_i64().unwrap(), "{ctx}: level");
        assert!(close(s.xp, f(&x["xp"])), "{ctx}: xp rust={} js={}", s.xp, x["xp"]);
        assert_eq!(s.need, x["need"].as_i64().unwrap(), "{ctx}: need");
    }
}

#[test]
fn small_rules() {
    assert_eq!(stage_enemy_count(1), 8);
    assert_eq!(stage_enemy_count(20), 21);
    assert_eq!(stage_enemy_count(100), 26);
    assert!(is_dungeon_boss_stage(10) && !is_dungeon_boss_stage(11));
    assert_eq!(kill_gold(100, 1.0), 45.0);
    assert_eq!(kill_xp(true), 120.0);
}
