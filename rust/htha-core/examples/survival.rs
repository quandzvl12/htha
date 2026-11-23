//! So sánh thống kê với bản JS: đứng yên ở ải ban đầu, đo thời điểm chết và số quái hạ được.
//! cargo run --release -p htha-core --example survival -- archer 1 300
use htha_core::sim::Sim;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (class, stage, n) = (a.get(1).map(String::as_str).unwrap_or("archer"), a.get(2).and_then(|x| x.parse().ok()).unwrap_or(1i64), a.get(3).and_then(|x| x.parse().ok()).unwrap_or(200u64));
    let (mut deaths, mut kills, mut survived) = (vec![], vec![], 0);
    for seed in 0..n {
        let mut s = Sim::new(class, seed, 360.0, 640.0).unwrap();
        s.stage = stage; s.player.max_hp += (stage - 1) as f64 * 100.0; s.player.hp = s.player.max_hp; s.start_stage();
        let mut t = 0.0;
        while t < 60.0 && !s.game_over { s.step(1.0 / 60.0, (0.0, 0.0)); t += 1.0 / 60.0; }
        if s.game_over { deaths.push(t); } else { survived += 1; }
        kills.push(s.kills as f64);
    }
    let mean = |v: &Vec<f64>| if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 };
    let mut d = deaths.clone(); d.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{class} ải {stage}: n={n} | chết trước 60s: {}/{} | thời điểm chết TB {:.1}s (trung vị {:.1}s, min {:.1}, max {:.1}) | quái hạ được TB {:.2}",
        deaths.len(), n, mean(&deaths), d.get(d.len() / 2).copied().unwrap_or(f64::NAN), d.first().copied().unwrap_or(f64::NAN), d.last().copied().unwrap_or(f64::NAN), mean(&kills));
    let _ = survived;
}
