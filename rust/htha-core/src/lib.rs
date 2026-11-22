//! htha-core — lõi logic của "Hành Trình Anh Hùng" viết bằng Rust.
//!
//! BƯỚC 1: công thức cân bằng (độ khó quái, vàng, giá bán, giá nâng cấp, ải/dungeon).
//! Mọi công thức được port nguyên văn từ `index.html` (khối `BAL` và các hàm liên quan);
//! kết quả được đối chiếu với JavaScript gốc trong `tests/golden.rs`.

pub mod balance;
pub mod dungeon;
pub mod ffi;
