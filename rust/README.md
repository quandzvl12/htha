# Chuyển "Hành Trình Anh Hùng" sang Rust — 5 bước, bước nào game cũng chạy được

| Bước | Nội dung | Trạng thái |
|---|---|---|
| 1 | Công thức cân bằng (độ khó, vàng, giá bán, giá nâng cấp, ải/dungeon) → `htha-core` (WASM) | ✅ xong |
| 2 | Dữ liệu game (quái, lớp nhân vật, độ hiếm, kỹ năng, tên) + lưu/đọc save + tài khoản | ⏳ |
| 3 | Mô phỏng: di chuyển, chiến đấu, AI quái, kỹ năng, luồng ải (tick cố định, JS chỉ vẽ) | ⏳ |
| 4 | Render + input + âm thanh + UI bằng Rust (macroquad/egui), chạy trong shell web | ⏳ |
| 5 | Bỏ WebView: đóng gói Rust thuần cho Android/iOS, multiplayer WebSocket | ⏳ |

## Chạy test (đối chiếu từng giá trị với công thức JS gốc)
    cd rust && cargo test --release

## Build WASM thủ công
    rustup target add wasm32-unknown-unknown
    cd rust && cargo build --release --target wasm32-unknown-unknown -p htha-core
    cp target/wasm32-unknown-unknown/release/htha_core.wasm ../htha_core.wasm   # đặt cạnh index.html khi chạy thử

## Sinh lại dữ liệu đối chiếu (sau khi đổi công thức JS)
    node rust/tools/gen_golden.js > rust/htha-core/tests/golden_data.rs

## Cách hoạt động ở bước 1
`index.html` có khối `Core` (cầu nối). Mặc định dùng bản JS dự phòng; khi `htha_core.wasm`
tải xong và khớp bản JS trên các giá trị mẫu thì tự chuyển sang Rust. Mất file wasm → game vẫn chạy bằng JS.
