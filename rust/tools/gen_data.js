// Sinh bảng dữ liệu Rust TỪ index.html (nguồn sự thật tới hết bước 3):
//   node rust/tools/gen_data.js > rust/htha-core/src/data_gen.rs
// CI chạy lại lệnh này và diff — đổi bảng trong index.html mà quên sinh lại thì CI báo lỗi.
const fs = require("fs");
const html = fs.readFileSync(__dirname + "/../../index.html", "utf8");
const grab = (re, what) => { const m = html.match(re); if (!m) throw new Error("Không thấy " + what); return m[1]; };
const RARITIES = eval(grab(/const RARITIES=(\[[\s\S]*?\n\s*\]);/, "RARITIES"));
const ENEMIES  = eval("(" + grab(/const enemyProfiles=(\{[\s\S]*?\n\s*\});/, "enemyProfiles") + ")");
const AVATARS  = eval(grab(/const PROFILE_AVATARS=(\[.*?\]);/, "PROFILE_AVATARS"));
const UNLOCK   = eval("(" + grab(/const PROFILE_UNLOCK_LEVELS=(\{.*?\});/, "PROFILE_UNLOCK_LEVELS") + ")");
// classes: trích đối tượng literal bằng cách đếm ngoặc
const cStart = html.indexOf("const classes={");
if (cStart < 0) throw new Error("Không thấy classes");
let depth = 0, cEnd = -1, inStr = null;
for (let i = cStart + "const classes=".length; i < html.length; i++) {
  const ch = html[i];
  if (inStr) { if (ch === "\\") i++; else if (ch === inStr) inStr = null; continue; }
  if (ch === '"' || ch === "'" || ch === "`") { inStr = ch; continue; }
  if (ch === "{") depth++; else if (ch === "}") { depth--; if (depth === 0) { cEnd = i + 1; break; } }
}
const CLASSES = eval("(" + html.slice(cStart + "const classes=".length, cEnd) + ")");
const q = (s) => JSON.stringify(s);
const f = (x) => { const s = String(x); return /[.e]/.test(s) ? s : s + ".0"; };
let o = "// TỰ SINH bởi rust/tools/gen_data.js từ index.html — đừng sửa tay.\n";
o += "pub struct Rarity { pub name: &'static str, pub color: &'static str, pub mult: f64 }\n";
o += "pub const RARITIES: &[Rarity] = &[\n" + RARITIES.map(r => `    Rarity { name: ${q(r.name)}, color: ${q(r.color)}, mult: ${f(r.mult)} },`).join("\n") + "\n];\n\n";
o += "pub struct EnemyProfile { pub key: &'static str, pub name: &'static str, pub r: f64, pub hp: f64, pub speed: f64, pub damage: f64, pub rate: f64, pub color: &'static str, pub is_boss: bool }\n";
o += "pub const ENEMY_PROFILES: &[EnemyProfile] = &[\n" + Object.entries(ENEMIES).map(([k, e]) =>
  `    EnemyProfile { key: ${q(k)}, name: ${q(e.name)}, r: ${f(e.r)}, hp: ${f(e.hp)}, speed: ${f(e.speed)}, damage: ${f(e.damage)}, rate: ${f(e.rate)}, color: ${q(e.color)}, is_boss: ${!!e.isBoss} },`).join("\n") + "\n];\n\n";
o += "pub const PROFILE_AVATARS: &[&str] = &[" + AVATARS.map(q).join(", ") + "];\n";
o += "pub const PROFILE_UNLOCK_LEVELS: &[(&str, i64)] = &[" + Object.entries(UNLOCK).map(([k, v]) => `(${q(k)}, ${v})`).join(", ") + "];\n";
o += "\npub struct ClassDef { pub key: &'static str, pub name: &'static str, pub hp: f64, pub mana: f64, pub mana_regen: f64, pub speed: f64, pub damage: f64, pub range: f64, pub rate: f64, pub melee: bool, pub god: bool }\n";
o += "pub const CLASSES: &[ClassDef] = &[\n" + Object.entries(CLASSES).map(([k, c]) =>
  `    ClassDef { key: ${q(k)}, name: ${q(c.name)}, hp: ${f(c.hp)}, mana: ${f(c.mana)}, mana_regen: ${f(c.manaRegen)}, speed: ${f(c.speed)}, damage: ${f(c.damage)}, range: ${f(c.range)}, rate: ${f(c.rate)}, melee: ${!!c.melee}, god: ${!!c.isGod} },`).join("\n") + "\n];\n";
process.stdout.write(o);
