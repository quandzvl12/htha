//! Đối chiếu Rust với công thức JavaScript gốc (dữ liệu sinh bằng rust/tools/gen_golden.js).
mod golden_data;
use golden_data::*;
use htha_core::{balance as b, dungeon as d, ffi};

fn close(a: f64, e: f64, what: &str) {
    let tol = 1e-12 * e.abs().max(1.0);
    assert!((a - e).abs() <= tol, "{what}: rust={a} js={e}");
}

#[test]
fn enemy_and_boss_scaling() {
    for (i, &s) in STAGES.iter().enumerate() {
        close(b::enemy_hp_mult(s), ENEMY_HP[i], &format!("enemy_hp ải {s}"));
        close(b::enemy_dmg_mult(s), ENEMY_DMG[i], &format!("enemy_dmg ải {s}"));
        close(b::boss_hp_mult(s, true), BOSS_HP[i], &format!("boss_hp ải {s}"));
        close(b::boss_dmg_mult(s, true), BOSS_DMG[i], &format!("boss_dmg ải {s}"));
        assert_eq!(b::boss_hp_mult(s, false), 1.0);
        assert_eq!(b::boss_dmg_mult(s, false), 1.0);
        close(b::gold_per_kill(s), GOLD[i], &format!("gold ải {s}"));
    }
}

#[test]
fn sell_values_match_exactly() {
    for c in SELL.chunks(4) {
        assert_eq!(b::sell_value(c[0], c[1], c[2]), c[3], "sell ải={} hiếm={} cường hóa={}", c[0], c[1], c[2]);
    }
}

#[test]
fn upgrade_costs_match_exactly() {
    for c in UPGRADE.chunks(3) {
        assert_eq!(b::upgrade_cost(c[0] as usize, c[1]), c[2], "upgrade kind={} lv={}", c[0], c[1]);
    }
}

#[test]
fn skill_values_match_exactly() {
    for c in SKILL.chunks(3) {
        assert_eq!(b::skill_value(c[0], c[1]), c[2], "skill b={} t={}", c[0], c[1]);
    }
}

#[test]
fn stage_to_dungeon_and_floor() {
    let from = STAGE_RANGE_FROM[0];
    for (i, s) in (from..from + DUNGEON_OF_STAGE.len() as i32).enumerate() {
        assert_eq!(d::dungeon_of_stage(s), DUNGEON_OF_STAGE[i], "dungeon ải {s}");
        assert_eq!(d::floor_of_stage(s), FLOOR_OF_STAGE[i], "floor ải {s}");
    }
}

#[test]
fn dungeon_names_and_ranks() {
    for n in 1..=300 {
        let i = (n - 1) as usize;
        assert_eq!(d::rank_index(n), RANK[i], "rank dungeon {n}");
        assert_eq!(d::region_index(n, REGIONS_LEN, ASPECTS_LEN), REGION[i], "region dungeon {n}");
        assert_eq!(d::aspect_index(n, ASPECTS_LEN), ASPECT[i], "aspect dungeon {n}");
    }
}

#[test]
fn ffi_layer_matches_core() {
    assert_eq!(ffi::core_version(), 1);
    assert_eq!(ffi::upgrade_cost(0, 10.0), b::upgrade_cost(0, 10.0));
    assert_eq!(ffi::boss_hp_mult(100.0, 1), b::boss_hp_mult(100.0, true));
    assert_eq!(ffi::boss_hp_mult(100.0, 0), 1.0);
    assert_eq!(ffi::sell_value(50.0, 5.0, 2.0), b::sell_value(50.0, 5.0, 2.0));
}
