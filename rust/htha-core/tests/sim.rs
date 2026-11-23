//! Kiểm thử mô phỏng thuần Rust: tính xác định, bất biến, và hành vi theo luật JS đã port.
use htha_core::combat::*;
use htha_core::sim::*;

fn run(class: &str, seed: u64, secs: f64, input: impl Fn(&Sim) -> (f64, f64)) -> Sim {
    let mut s = Sim::new(class, seed, 360.0, 640.0).unwrap();
    let dt = 1.0 / 60.0;
    for _ in 0..(secs / dt) as usize {
        let i = input(&s);
        s.step(dt, i);
        if s.game_over { break; }
    }
    s
}

#[test]
fn deterministic_for_same_seed() {
    let a = run("archer", 7, 40.0, |_| (0.0, 0.0));
    let b = run("archer", 7, 40.0, |_| (0.0, 0.0));
    assert_eq!((a.stage, a.kills, a.xp.level), (b.stage, b.kills, b.xp.level));
    assert_eq!(a.player.hp.to_bits(), b.player.hp.to_bits());
    let c = run("archer", 8, 40.0, |_| (0.0, 0.0));
    assert!(a.enemies.iter().zip(&c.enemies).any(|(x, y)| x.e.x != y.e.x) || a.kills != c.kills);
}

#[test]
fn stage_spawns_expected_count_and_boss() {
    let s = Sim::new("archer", 1, 360.0, 640.0).unwrap();
    assert!(s.enemies.len() as i64 >= stage_enemy_count(1));
    let mut s = Sim::new("archer", 1, 360.0, 640.0).unwrap();
    s.stage = 10; s.start_stage();
    assert_eq!(s.enemies.iter().filter(|e| e.e.kind == "dinhhanh").count(), 1);
    assert_eq!(s.enemies.len() as i64, stage_enemy_count(10));
}

#[test]
fn archer_progresses_and_gains_rewards() {
    let s = run("archer", 3, 60.0, |_| (0.0, 0.0));
    assert!(s.kills > 0, "phải hạ được quái");
    assert!(s.coins > 0.0 && s.gems > 0);
    assert!(s.xp.level >= 2 || s.xp.xp > 0.0);
}

#[test]
fn invariants_hold_over_long_run() {
    let mut s = Sim::new("warrior", 11, 360.0, 640.0).unwrap();
    for i in 0..20000 {
        let t = i as f64 / 60.0;
        s.step(1.0 / 60.0, ((t * 0.7).sin(), (t * 0.5).cos()));
        let p = &s.player;
        assert!(p.x >= 25.0 && p.x <= WW - 25.0 && p.y >= 40.0 && p.y <= WH - 40.0, "người chơi ra ngoài bản đồ");
        assert!(p.hp.is_finite() && p.hp <= p.max_hp + 1e-9 && p.hp >= 0.0);
        for e in &s.enemies { assert!(e.e.x.is_finite() && e.e.y.is_finite() && e.e.hp.is_finite()); }
        assert!(s.shots.len() < 500 && s.eshots.len() < 2000, "rò rỉ đạn");
        if s.game_over { break; }
    }
}

#[test]
fn player_can_die_without_upgrades_at_high_stage() {
    let mut s = Sim::new("mage", 5, 360.0, 640.0).unwrap();
    s.stage = 400; s.start_stage();
    for _ in 0..(60 * 120) { s.step(1.0 / 60.0, (0.0, 0.0)); if s.game_over { break; } }
    assert!(s.game_over, "ải 400 không nâng cấp phải thua");
    assert!(s.take_events().iter().any(|e| matches!(e, Event::GameOver)));
}

#[test]
fn contact_damage_respects_enemy_cooldown() {
    let mut s = Sim::new("warrior", 2, 360.0, 640.0).unwrap();
    s.enemies.truncate(1);
    s.last_shot = 1e9; // tắt tự đánh để chỉ kiểm tra va chạm
    let e = &mut s.enemies[0];
    e.e.x = s.player.x + 10.0; e.e.y = s.player.y; e.e.last_attack = 1e9; e.e.hp = 1e12; e.e.max_hp = 1e12;
    let dmg = e.e.damage * e.e.damage_out_mult;
    let hp0 = s.player.hp;
    s.step(1.0 / 60.0, (0.0, 0.0));
    assert!((hp0 - s.player.hp - dmg).abs() < 1e-9, "va chạm đầu tiên gây đúng sát thương");
    let hp1 = s.player.hp;
    s.step(1.0 / 60.0, (0.0, 0.0));
    assert_eq!(s.player.hp, hp1, "trong thời gian hồi chiêu (rate/1000 s) không bị đánh lần nữa");
}

#[test]
fn melee_hit_suppresses_contact_damage_like_js_shared_hit_field() {
    // JS dùng chung `e.hit` cho nháy-khi-bị-đánh và hồi-chiêu-va-chạm: bị chém thì 0,16 s không đánh lại.
    let mut s = Sim::new("warrior", 2, 360.0, 640.0).unwrap();
    s.enemies.truncate(1);
    let e = &mut s.enemies[0];
    e.e.x = s.player.x + 10.0; e.e.y = s.player.y; e.e.last_attack = 1e9; e.e.hp = 1e12; e.e.max_hp = 1e12;
    let hp0 = s.player.hp;
    s.step(1.0 / 60.0, (0.0, 0.0));
    assert_eq!(s.player.hp, hp0, "bị chém → không gây sát thương va chạm trong khung này");
}

#[test]
fn unknown_and_god_class_rejected() {
    assert!(Sim::new("khong-co", 1, 360.0, 640.0).is_none());
    assert!(Sim::new("god", 1, 360.0, 640.0).is_none());
}
