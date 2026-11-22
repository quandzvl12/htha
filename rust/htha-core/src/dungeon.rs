//! Ải / dungeon / bậc — tương đương `dungeonName`, `dungeonRank`, `currentStageName`.
//! Bảng tên (tiếng Việt) vẫn nằm ở JS tới bước 2; Rust chỉ trả về chỉ số.

pub const MAX_DUNGEONS: i32 = 300;
pub const STAGES_PER_DUNGEON: i32 = 10;

/// Dungeon của một ải: `clamp(ceil(ải / 10), 1, 300)`.
pub fn dungeon_of_stage(stage: i32) -> i32 {
    let d = (stage as f64 / STAGES_PER_DUNGEON as f64).ceil() as i32;
    d.clamp(1, MAX_DUNGEONS)
}

/// Số ải trong dungeon (1..=10): `(ải - 1) % 10 + 1` — `%` giữ nguyên ngữ nghĩa của JS.
pub fn floor_of_stage(stage: i32) -> i32 {
    (stage - 1) % STAGES_PER_DUNGEON + 1
}

/// Chỉ số bậc: 0 Tân Thủ, 1 Đồng, 2 Bạc, 3 Vàng, 4 Kim Cương, 5 Huyền Thoại, 6 Thần Thoại, 7 Huỷ Diệt.
pub fn rank_index(dungeon: i32) -> i32 {
    match dungeon {
        1 => 0,
        d if d <= 20 => 1,
        d if d <= 60 => 2,
        d if d <= 120 => 3,
        d if d <= 180 => 4,
        d if d <= 240 => 5,
        d if d <= 280 => 6,
        _ => 7,
    }
}

/// Chỉ số vùng (REGIONS) của dungeon từ số 2 trở đi; dungeon 1 là "Tân Thủ" (trả -1).
pub fn region_index(dungeon: i32, regions_len: i32, aspects_len: i32) -> i32 {
    if dungeon == 1 || regions_len <= 0 || aspects_len <= 0 {
        return -1;
    }
    let index = dungeon - 2;
    // floor-division kiểu JS (index có thể âm nếu dungeon < 1)
    let q = (index as f64 / aspects_len as f64).floor() as i32;
    q.rem_euclid(regions_len)
}

/// Chỉ số hệ (ASPECTS) của dungeon từ số 2 trở đi; dungeon 1 trả -1.
pub fn aspect_index(dungeon: i32, aspects_len: i32) -> i32 {
    if dungeon == 1 || aspects_len <= 0 {
        return -1;
    }
    (dungeon - 2).rem_euclid(aspects_len)
}
