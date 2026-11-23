//! htha-core — lõi logic của "Hành Trình Anh Hùng" viết bằng Rust.
//!
//! BƯỚC 1: công thức cân bằng (độ khó quái, vàng, giá bán, giá nâng cấp, ải/dungeon).
//! BƯỚC 2: bảng dữ liệu (độ hiếm, quái), hồ sơ người chơi và quản lý save theo nghề.
//!
//! Mọi công thức được port nguyên văn từ `index.html` (khối `BAL` và các hàm liên quan);
//! kết quả được đối chiếu với JavaScript gốc trong `tests/golden.rs`.

pub mod balance;
pub mod combat;
pub mod data_gen;
pub mod dungeon;
pub mod ffi;
pub mod jsnum;
pub mod profile;
pub mod saves;
pub mod sim;
