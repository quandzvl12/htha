// TỰ SINH bởi rust/tools/gen_data.js từ index.html — đừng sửa tay.
pub struct Rarity { pub name: &'static str, pub color: &'static str, pub mult: f64 }
pub const RARITIES: &[Rarity] = &[
    Rarity { name: "Thường", color: "#b8b8c8", mult: 1.0 },
    Rarity { name: "Hiếm", color: "#4fc3ff", mult: 2.4 },
    Rarity { name: "Sử Thi", color: "#c061ff", mult: 5.0 },
    Rarity { name: "Huyền Thoại", color: "#ffb020", mult: 10.0 },
    Rarity { name: "Thần Thoại", color: "#ff4466", mult: 20.0 },
];

pub struct EnemyProfile { pub key: &'static str, pub name: &'static str, pub r: f64, pub hp: f64, pub speed: f64, pub damage: f64, pub rate: f64, pub color: &'static str, pub is_boss: bool }
pub const ENEMY_PROFILES: &[EnemyProfile] = &[
    EnemyProfile { key: "duc", name: "Đức", r: 15.0, hp: 38.0, speed: 48.0, damage: 7.0, rate: 1700.0, color: "#e9dfd2", is_boss: false },
    EnemyProfile { key: "tayhung", name: "Tày Hùng", r: 23.0, hp: 118.0, speed: 32.0, damage: 15.0, rate: 2100.0, color: "#d49d50", is_boss: false },
    EnemyProfile { key: "dinhhanh", name: "Dinh Hanh", r: 35.0, hp: 520.0, speed: 25.0, damage: 28.0, rate: 1250.0, color: "#66717d", is_boss: true },
    EnemyProfile { key: "quan", name: "Quân", r: 17.0, hp: 68.0, speed: 58.0, damage: 10.0, rate: 1450.0, color: "#744b8d", is_boss: false },
    EnemyProfile { key: "duy", name: "Duy", r: 21.0, hp: 104.0, speed: 40.0, damage: 13.0, rate: 1850.0, color: "#71a94e", is_boss: false },
    EnemyProfile { key: "tam", name: "Tâm", r: 22.0, hp: 135.0, speed: 34.0, damage: 14.0, rate: 2100.0, color: "#4e82aa", is_boss: false },
    EnemyProfile { key: "nghia", name: "Nghĩa", r: 20.0, hp: 108.0, speed: 39.0, damage: 12.0, rate: 1900.0, color: "#6d9e4c", is_boss: false },
    EnemyProfile { key: "thanh", name: "Thành", r: 24.0, hp: 160.0, speed: 30.0, damage: 18.0, rate: 2200.0, color: "#8b9298", is_boss: false },
];

pub const PROFILE_AVATARS: &[&str] = &["🧑‍🚀", "🛡️", "🏹", "🪄", "⚔️", "🐺", "🟣", "🐉", "🌟", "🦊"];
pub const PROFILE_UNLOCK_LEVELS: &[(&str, i64)] = &[("hero", 20), ("hakaishin", 50)];

pub struct ClassDef { pub key: &'static str, pub name: &'static str, pub hp: f64, pub mana: f64, pub mana_regen: f64, pub speed: f64, pub damage: f64, pub range: f64, pub rate: f64, pub melee: bool, pub god: bool }
pub const CLASSES: &[ClassDef] = &[
    ClassDef { key: "warrior", name: "Chiến Binh", hp: 150.0, mana: 80.0, mana_regen: 3.2, speed: 145.0, damage: 58.0, range: 88.0, rate: 520.0, melee: false, god: false },
    ClassDef { key: "archer", name: "Cung Thủ", hp: 105.0, mana: 100.0, mana_regen: 4.6, speed: 170.0, damage: 17.0, range: 285.0, rate: 470.0, melee: false, god: false },
    ClassDef { key: "mage", name: "Pháp Sư", hp: 85.0, mana: 150.0, mana_regen: 7.2, speed: 130.0, damage: 23.0, range: 265.0, rate: 720.0, melee: false, god: false },
    ClassDef { key: "hero", name: "Anh Hùng", hp: 170.0, mana: 90.0, mana_regen: 3.6, speed: 150.0, damage: 40.0, range: 88.0, rate: 560.0, melee: true, god: false },
    ClassDef { key: "summoner", name: "Triệu Hồi Sư", hp: 95.0, mana: 130.0, mana_regen: 5.4, speed: 135.0, damage: 14.0, range: 260.0, rate: 680.0, melee: false, god: false },
    ClassDef { key: "goon", name: "Tâm Goon", hp: 110.0, mana: 120.0, mana_regen: 5.0, speed: 140.0, damage: 20.0, range: 300.0, rate: 420.0, melee: false, god: false },
    ClassDef { key: "slime", name: "Slime", hp: 80.0, mana: 120.0, mana_regen: 6.0, speed: 130.0, damage: 12.0, range: 70.0, rate: 800.0, melee: true, god: false },
    ClassDef { key: "hakaishin", name: "Thần Huỷ Diệt (Hakaishin)", hp: 170.0, mana: 150.0, mana_regen: 7.0, speed: 145.0, damage: 46.0, range: 88.0, rate: 520.0, melee: true, god: false },
    ClassDef { key: "god", name: "Đấng Tối Cao", hp: 9999999.0, mana: 99999.0, mana_regen: 99999.0, speed: 180.0, damage: 9999999.0, range: 9999.0, rate: 120.0, melee: false, god: true },
];
