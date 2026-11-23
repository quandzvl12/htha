//! Hành Trình Anh Hùng — bản Rust thuần (lát cắt bước 4a).
//! Lõi luật chơi nằm trong `htha-core` (đã đối chiếu với JS); file này chỉ lo vẽ, nhập liệu và vòng lặp.
//! Chạy thử trên máy tính:  cargo run --release -p htha-game
//! Điều khiển: WASD / phím mũi tên, hoặc kéo chuột / ngón tay (joystick ảo). Tự động đánh quái gần nhất.

use htha_core::combat::{WH, WW};
use htha_core::sim::{Event, Sim};
use macroquad::prelude::*;

/// Kích thước màn hình logic giống bản JS (360×640); mọi thứ được scale theo chiều cao màn hình thật.
const LOGICAL_H: f32 = 640.0;
const STEP: f64 = 1.0 / 60.0;
const CLASSES: [&str; 5] = ["warrior", "archer", "mage", "hero", "summoner"];

fn conf() -> Conf {
    Conf { window_title: "Hanh Trinh Anh Hung (Rust)".to_owned(), window_width: 450, window_height: 800, high_dpi: true, ..Default::default() }
}

fn hex(c: &str) -> Color {
    let c = c.trim_start_matches('#');
    let v = |i: usize| u8::from_str_radix(c.get(i..i + 2).unwrap_or("ff"), 16).unwrap_or(255) as f32 / 255.0;
    if c.len() >= 6 { Color::new(v(0), v(2), v(4), 1.0) } else { WHITE }
}

fn class_color(key: &str) -> Color {
    match key {
        "warrior" => hex("#f29b62"), "archer" => hex("#75d882"), "mage" => hex("#a887ed"),
        "hero" => hex("#ffd45c"), "summoner" => hex("#8ad6ff"), _ => WHITE,
    }
}

struct Joystick { origin: Option<Vec2>, cur: Vec2 }
impl Joystick {
    /// Gộp bàn phím + chuột + cảm ứng thành một vectơ hướng (−1..1).
    fn read(&mut self) -> (f64, f64) {
        let mut k = vec2(0.0, 0.0);
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { k.x -= 1.0; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { k.x += 1.0; }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { k.y -= 1.0; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { k.y += 1.0; }
        if k.length() > 0.0 { let n = k.normalize(); return (n.x as f64, n.y as f64); }
        let pointer = touches().iter().find(|t| t.phase != TouchPhase::Ended && t.phase != TouchPhase::Cancelled).map(|t| t.position)
            .or(if is_mouse_button_down(MouseButton::Left) { Some(mouse_position().into()) } else { None });
        match pointer {
            Some(p) => { let o = *self.origin.get_or_insert(p); self.cur = p;
                let d = (p - o) / (screen_height() * 0.09); let l = d.length();
                if l > 0.15 { let d = if l > 1.0 { d / l } else { d }; (d.x as f64, d.y as f64) } else { (0.0, 0.0) } }
            None => { self.origin = None; (0.0, 0.0) }
        }
    }
}

fn draw_bar(x: f32, y: f32, w: f32, h: f32, frac: f32, col: Color) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.0, 0.0, 0.55));
    draw_rectangle(x, y, w * frac.clamp(0.0, 1.0), h, col);
}

fn draw_world(sim: &Sim, sc: f32, shake: Vec2) {
    let (sw, sh) = (screen_width(), screen_height());
    let cam = vec2((sim.player.x as f32 - sw / sc / 2.0).clamp(-70.0, WW as f32 - sw / sc + 70.0),
                   (sim.player.y as f32 - sh / sc * 0.5).clamp(-70.0, WH as f32 - sh / sc + 70.0));
    let tx = |x: f64| (x as f32 - cam.x) * sc + shake.x;
    let ty = |y: f64| (y as f32 - cam.y) * sc + shake.y;
    clear_background(hex("#14121f"));
    // lưới nền + viền thế giới
    let g = 100.0;
    let (x0, y0) = ((cam.x / g).floor() * g, (cam.y / g).floor() * g);
    let mut x = x0;
    while x < cam.x + sw / sc + g { draw_line(tx(x as f64), 0.0, tx(x as f64), sh, 1.0, Color::new(1.0, 1.0, 1.0, 0.045)); x += g; }
    let mut y = y0;
    while y < cam.y + sh / sc + g { draw_line(0.0, ty(y as f64), sw, ty(y as f64), 1.0, Color::new(1.0, 1.0, 1.0, 0.045)); y += g; }
    draw_rectangle_lines(tx(0.0), ty(0.0), WW as f32 * sc, WH as f32 * sc, 4.0, hex("#8a4bd6"));

    for e in sim.enemies.iter().filter(|e| !e.dead) {
        let (x, y, r) = (tx(e.e.x), ty(e.e.y), e.e.r as f32 * sc);
        if x < -80.0 || y < -80.0 || x > sw + 80.0 || y > sh + 80.0 { continue; }
        let col = if e.hit > 0.0 && e.hit < 0.2 { WHITE } else { hex(e.e.color) };
        draw_circle(x, y, r, col);
        draw_circle_lines(x, y, r, 2.0, if e.e.is_boss { hex("#e83e59") } else if e.e.elite { hex("#ffd45c") } else { Color::new(0.0, 0.0, 0.0, 0.5) });
        draw_bar(x - r, y - r - 8.0 * sc, r * 2.0, 4.0 * sc, (e.e.hp / e.e.max_hp) as f32, if e.e.is_boss { hex("#e83e59") } else { hex("#ff7583") });
        if e.e.is_boss { draw_text(e.e.kind, x - r, y - r - 12.0 * sc, 16.0 * sc, WHITE); }
    }
    for s in &sim.shots { draw_circle(tx(s.x), ty(s.y), s.r as f32 * sc, hex("#b8f283")); }
    for s in &sim.eshots { draw_circle(tx(s.x), ty(s.y), s.r as f32 * sc, if s.boss { hex("#e83e59") } else { hex("#ff7583") }); }
    let p = &sim.player;
    let pc = if p.flash > 0.0 { hex("#ff5c6e") } else { class_color(sim.class.key) };
    draw_circle(tx(p.x), ty(p.y), p.r as f32 * sc, pc);
    draw_circle_lines(tx(p.x), ty(p.y), p.r as f32 * sc, 2.0, WHITE);
}

fn draw_hud(sim: &Sim, sc: f32) {
    let w = screen_width();
    let u = sc;
    draw_bar(10.0 * u, 12.0 * u, w - 20.0 * u, 12.0 * u, (sim.player.hp / sim.player.max_hp) as f32, hex("#e8465f"));
    draw_text(&format!("HP {:.0}/{:.0}", sim.player.hp, sim.player.max_hp), 14.0 * u, 22.0 * u, 12.0 * u, WHITE);
    draw_bar(10.0 * u, 28.0 * u, w - 20.0 * u, 6.0 * u, (sim.xp.xp / sim.xp.need as f64) as f32, hex("#8a4bd6"));
    draw_text(&format!("Stage {}   Lv {}   Kills {}   Gold {:.0}   Gems {}", sim.stage, sim.xp.level, sim.kills, sim.coins, sim.gems),
        10.0 * u, 50.0 * u, 14.0 * u, WHITE);
    draw_text(&format!("{} FPS", get_fps()), w - 60.0 * u, 50.0 * u, 12.0 * u, Color::new(1.0, 1.0, 1.0, 0.5));
}

enum Screen { Menu, Playing(Box<Sim>), Over(Box<Sim>) }

#[macroquad::main(conf)]
async fn main() {
    let mut screen = Screen::Menu;
    let mut joy = Joystick { origin: None, cur: vec2(0.0, 0.0) };
    let mut acc = 0.0f64;
    let mut shake = 0.0f32;
    let mut seed = 1u64;
    loop {
        let sc = screen_height() / LOGICAL_H;
        clear_background(hex("#14121f"));
        match &mut screen {
            Screen::Menu => {
                draw_text("HANH TRINH ANH HUNG", 20.0 * sc, 90.0 * sc, 30.0 * sc, WHITE);
                draw_text("Ban Rust thuan (lat cat buoc 4a)", 20.0 * sc, 115.0 * sc, 15.0 * sc, Color::new(1.0, 1.0, 1.0, 0.6));
                let mut chosen: Option<&str> = None;
                for (i, key) in CLASSES.iter().enumerate() {
                    let (x, y, w, h) = (20.0 * sc, (150.0 + i as f32 * 70.0) * sc, screen_width() - 40.0 * sc, 56.0 * sc);
                    draw_rectangle(x, y, w, h, Color::new(1.0, 1.0, 1.0, 0.07));
                    draw_rectangle(x, y, 8.0 * sc, h, class_color(key));
                    let c = htha_core::combat::class_def(key).unwrap();
                    draw_text(&format!("{}  {}", i + 1, key.to_uppercase()), x + 20.0 * sc, y + 24.0 * sc, 20.0 * sc, WHITE);
                    draw_text(&format!("HP {}  DMG {}  SPD {}  RANGE {}", c.hp, c.damage, c.speed, c.range), x + 20.0 * sc, y + 44.0 * sc, 13.0 * sc, Color::new(1.0, 1.0, 1.0, 0.65));
                    let key_pressed = [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5][i];
                    let tapped = touches().iter().any(|t| t.phase == TouchPhase::Started && t.position.y >= y && t.position.y <= y + h)
                        || (is_mouse_button_pressed(MouseButton::Left) && mouse_position().1 >= y && mouse_position().1 <= y + h);
                    if is_key_pressed(key_pressed) || tapped { chosen = Some(key); }
                }
                draw_text("Chon nghe: bam phim 1-5 hoac cham vao the", 20.0 * sc, 540.0 * sc, 14.0 * sc, Color::new(1.0, 1.0, 1.0, 0.5));
                if let Some(k) = chosen {
                    seed = seed.wrapping_add((get_time() * 1000.0) as u64);
                    let vw = (screen_width() / sc) as f64;
                    if let Some(s) = Sim::new(k, seed, vw, LOGICAL_H as f64) { screen = Screen::Playing(Box::new(s)); acc = 0.0; }
                }
            }
            Screen::Playing(sim) => {
                acc += (get_frame_time() as f64).min(0.1);
                let input = joy.read();
                while acc >= STEP && !sim.game_over { sim.step(STEP, input); acc -= STEP; }
                for ev in sim.take_events() {
                    match ev { Event::PlayerHurt { .. } => shake = 6.0, Event::EnemyKilled { boss: true, .. } => shake = 10.0, _ => {} }
                }
                shake *= 0.85;
                let sh = vec2((rand::gen_range(-1.0, 1.0)) * shake, (rand::gen_range(-1.0, 1.0)) * shake);
                draw_world(sim, sc, sh);
                draw_hud(sim, sc);
                if let Some(o) = joy.origin { draw_circle(o.x, o.y, 38.0 * sc, Color::new(1.0, 1.0, 1.0, 0.12)); draw_circle(joy.cur.x, joy.cur.y, 16.0 * sc, Color::new(1.0, 1.0, 1.0, 0.3)); }
                if sim.game_over { let s = std::mem::replace(sim, Box::new(Sim::new("archer", 0, 360.0, 640.0).unwrap())); screen = Screen::Over(s); }
            }
            Screen::Over(sim) => {
                draw_world(sim, sc, vec2(0.0, 0.0));
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.65));
                draw_text("HET MAU", 20.0 * sc, 250.0 * sc, 40.0 * sc, hex("#e8465f"));
                draw_text(&format!("Stage {}  Lv {}  Kills {}  Gold {:.0}", sim.stage, sim.xp.level, sim.kills, sim.coins), 20.0 * sc, 285.0 * sc, 16.0 * sc, WHITE);
                draw_text("Cham de choi lai", 20.0 * sc, 330.0 * sc, 18.0 * sc, Color::new(1.0, 1.0, 1.0, 0.7));
                if is_mouse_button_pressed(MouseButton::Left) || touches().iter().any(|t| t.phase == TouchPhase::Started) || is_key_pressed(KeyCode::Space) { screen = Screen::Menu; }
            }
        }
        next_frame().await
    }
}
