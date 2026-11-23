# Chuyển "Hành Trình Anh Hùng" sang Rust — 5 bước, bước nào game cũng chạy được

| Bước | Nội dung | Trạng thái |
|---|---|---|
| 1 | Công thức cân bằng (độ khó, vàng, giá bán, giá nâng cấp, ải/dungeon) → `htha-core` (WASM) | ✅ xong |
| 2 | Bảng dữ liệu (độ hiếm, quái) + hồ sơ người chơi + ghi/xóa save theo nghề (chuỗi/JSON qua WASM) | ✅ xong |
| 3 | **Tối ưu hiệu năng** (đo thật → sprite cache Anh Linh: −87% lệnh vẽ ở Q0). Mô phỏng chuyển Rust: bỏ, vì chỉ tốn ~0,2 ms/khung | ✅ xong |
| 4a | **Lõi chiến đấu + bản game Rust chạy được** (macroquad: desktop/WASM): sinh quái, chỉ số, XP, thưởng, di chuyển, bắn, va chạm, chuyển ải | ✅ lát cắt xong |
| 4b | Phần còn lại của game: kỹ năng, trang bị, boss (80+ chiêu), triệu hồi/Anh Linh, chợ, hiệu ứng, mô hình quái/nhân vật, UI tiếng Việt, âm thanh, lưu game | ⏳ |
| 5 | Đóng gói Android/iOS cho bản Rust, multiplayer WebSocket, thay bản JS | ⏳ |

## Chạy test (đối chiếu từng giá trị với công thức JS gốc)
    cd rust && cargo test --release

## Build WASM thủ công
    rustup target add wasm32-unknown-unknown
    cd rust && cargo build --release --target wasm32-unknown-unknown -p htha-core
    cp target/wasm32-unknown-unknown/release/htha_core.wasm ../htha_core.wasm   # đặt cạnh index.html khi chạy thử

## Sinh lại dữ liệu đối chiếu (sau khi đổi công thức JS)
    node rust/tools/gen_golden.js > rust/htha-core/tests/golden_data.rs

## Bước 2 — chạy đối chiếu
    node rust/tools/gen_golden2.js     # chạy CHÍNH hàm JS gốc, sinh dữ liệu đối chiếu (612 hồ sơ, 2.022 thao tác save)
    node rust/tools/apply_step2.py     # (python3) vá index.html; chạy sau apply_step1.py
Bảng dữ liệu Rust sinh từ index.html: `node rust/tools/gen_data.js > rust/htha-core/src/data_gen.rs`.

## Cách hoạt động ở bước 1
`index.html` có khối `Core` (cầu nối). Mặc định dùng bản JS dự phòng; khi `htha_core.wasm`
tải xong và khớp bản JS trên các giá trị mẫu thì tự chuyển sang Rust. Mất file wasm → game vẫn chạy bằng JS.

## Bước 3 — đo hiệu năng & tối ưu
Công cụ đo nằm ở `rust/tools/perf/` (jsdom + canvas giả đếm lệnh, đồng hồ ảo, giả lập Android). Kết quả chính:
- `update()` (mô phỏng) chỉ ~0,15–0,2 ms/khung → chuyển sang Rust không giúp gì.
- `draw()` mới nặng. Điểm nóng: 3 Anh Linh (Enkidu/Gilgamesh/Artoria) ~2.000–6.000 lệnh canvas/khung/con, kể cả ở Q0.
- Sửa: `drawServantModel` vẽ Anh Linh vào canvas phụ ~15 lần/giây rồi `drawImage`. Tắt bằng `localStorage.htahNoSvCache=1`.
- Kiểm thử: `npm i --no-save @napi-rs/canvas && node rust/tools/test_servant_cache.js` (so từng điểm ảnh với vẽ trực tiếp).
- Vá: `python3 rust/tools/apply_step3.py` (sau apply_step1/2).

## Bước 4a — bản game Rust (`rust/htha-game`)
    cargo run --release -p htha-game          # desktop; phím 1-5 chọn nghề, WASD/mũi tên hoặc kéo chuột để di chuyển
    cargo build --release -p htha-game --target wasm32-unknown-unknown   # web (CI đóng gói kèm mq_js_bundle.js)

**Đã đối chiếu từng giá trị với hàm JS gốc** (`tests/golden_combat.rs`, dữ liệu sinh bằng `tools/gen_golden3.js`):
chỉ số quái (`spawnEnemy`, 418 ca gồm boss/boss ẩn/tinh anh), chỉ số người chơi (`applyClass`, 384 ca), XP/lên cấp (`gainXP`, 144 ca).
**Đã so sánh thống kê** (`examples/survival.rs`): đứng yên ở ải 1 — JS chết ~4,3 s, Rust ~3,8 s (JS đo theo giây nguyên).
**Mô phỏng di chuyển/va chạm/bắn** port từ vòng lặp `update()` nhưng chỉ kiểm bằng test hành vi + so thống kê, không phải từng khung.

**CHƯA có so với bản JS:** kỹ năng, trang bị, nâng cấp trong chợ, chiêu boss, triệu hồi & Anh Linh, thú cưng, chợ, hiệu ứng hạt,
mô hình quái/nhân vật (đang là hình tròn), chữ tiếng Việt (font mặc định chỉ có ASCII), âm thanh, lưu game, multiplayer.
Bản JS vẫn là bản chính cho tới khi 4b đạt tương đương.

**Android/iOS (CHƯA kiểm chứng ở đây):** `cargo install cargo-quad-apk` rồi `cargo quad-apk build --release -p htha-game`
(cần Android SDK/NDK; metadata nằm trong `htha-game/Cargo.toml`). iOS theo hướng dẫn của macroquad (Xcode).
