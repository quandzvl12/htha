// Sinh dữ liệu "vàng" từ CÔNG THỨC JAVASCRIPT GỐC (chép nguyên văn từ index.html)
// để test Rust đối chiếu từng giá trị. Chạy: node rust/tools/gen_golden.js > rust/htha-core/tests/golden_data.rs
const fs = require("fs");
const html = fs.readFileSync(__dirname + "/../../index.html", "utf8");
const arrLen = (name) => {
  const m = html.match(new RegExp("const " + name + "=\\[(.*?)\\];", "s"));
  return eval("[" + m[1] + "]").length;
};
const REG = arrLen("DUNGEON_REGIONS"), ASP = arrLen("DUNGEON_ASPECTS");

// ---- nguyên văn từ index.html ----
const BAL={goldPerKill:30,goldStageK:0.005,sellBase:300,sellStageK:0.02,sellExp:0.9,skillScale:0.6,
  enemyHpA:0.1244,enemyHpB:1.2144,enemyDmgA:0.2093,enemyDmgB:1.4806,
  bossHpA:7.75,bossHpB:-0.811,bossDmgA:4.199,bossDmgB:-0.364};
const UPGRADE_BASE={power:20,speed:20,health:20},UP_COST_PCT=0.20;
const MAX_DUNGEONS=300,STAGES_PER_DUNGEON=10;
function dungeonRank(number){if(number===1)return"Tân Thủ";if(number<=20)return"Đồng";if(number<=60)return"Bạc";if(number<=120)return"Vàng";if(number<=180)return"Kim Cương";if(number<=240)return"Huyền Thoại";if(number<=280)return"Thần Thoại";return"Huỷ Diệt"}
const RANKS=["Tân Thủ","Đồng","Bạc","Vàng","Kim Cương","Huyền Thoại","Thần Thoại","Huỷ Diệt"];
const upgradeCost=(kind,lv)=>Math.max(1,Math.round(UPGRADE_BASE[kind]*Math.pow(1+UP_COST_PCT,lv)));
const p=(b,t)=>Math.max(1,Math.round(b*t*BAL.skillScale));
// ----------------------------------

const f = (x) => { const s = String(x); return /[.e]/.test(s) ? s : s + ".0"; };
const fa = (name, arr) => `pub const ${name}: &[f64] = &[${arr.map(f).join(", ")}];\n`;
const ia = (name, arr) => `pub const ${name}: &[i32] = &[${arr.join(", ")}];\n`;

const S = [1,2,3,5,9,10,11,25,50,99,100,101,250,500,777,1000,1500,2999,3000];
let out = "// TỰ SINH bởi rust/tools/gen_golden.js từ công thức JS gốc — đừng sửa tay.\n";
out += `pub const REGIONS_LEN: i32 = ${REG};\npub const ASPECTS_LEN: i32 = ${ASP};\n`;
out += fa("STAGES", S);
out += fa("ENEMY_HP", S.map(s=>Math.max(1,BAL.enemyHpA*Math.pow(s,BAL.enemyHpB))));
out += fa("ENEMY_DMG", S.map(s=>Math.max(1,BAL.enemyDmgA*Math.pow(s,BAL.enemyDmgB))));
out += fa("BOSS_HP", S.map(s=>Math.max(.0001,BAL.bossHpA*Math.pow(s,BAL.bossHpB))));
out += fa("BOSS_DMG", S.map(s=>Math.max(.0001,BAL.bossDmgA*Math.pow(s,BAL.bossDmgB))));
out += fa("GOLD", S.map(s=>BAL.goldPerKill*(1+s*BAL.goldStageK)));

// (stage, rarityMult, enhance, expected) — công thức 5352 (genWeapon dùng enhance=0 cho cùng kết quả)
const sell = [];
for (const s of S) for (const r of [1,2.4,5,10,20]) for (const e of [0,3]) {
  const stageFactor = 1 + Math.max(0, s) * BAL.sellStageK, enh = 1 + e * 0.25;
  sell.push([s, r, e, Math.round(BAL.sellBase*stageFactor*Math.pow(r,BAL.sellExp)*enh)]);
}
out += fa("SELL", sell.flat());

const up = [];
for (const k of ["power","speed","health"]) for (const lv of [0,1,5,10,25,50,100]) up.push([["power","speed","health"].indexOf(k), lv, upgradeCost(k, lv)]);
out += fa("UPGRADE", up.flat());

const sk = [];
for (const b of [0.2,3,10,25.5,100,1234]) for (const t of [1,1.05,1.5,2.35]) sk.push([b, t, p(b, t)]);
out += fa("SKILL", sk.flat());

// ải 0..3010: dungeon, floor
const st = [], dg = [], fl = [];
for (let s = -3; s <= 3010; s++) { st.push(s); dg.push(Math.min(MAX_DUNGEONS,Math.max(1,Math.ceil(s/STAGES_PER_DUNGEON)))); fl.push((s-1)%STAGES_PER_DUNGEON+1); }
out += ia("STAGE_RANGE_FROM", [st[0]]);
out += ia("DUNGEON_OF_STAGE", dg);
out += ia("FLOOR_OF_STAGE", fl);

// dungeon 1..300: rank / region / aspect
const rk = [], rg = [], as = [];
for (let d = 1; d <= 300; d++) {
  rk.push(RANKS.indexOf(dungeonRank(d)));
  const index = d - 2;
  if (d === 1) { rg.push(-1); as.push(-1); }
  else { rg.push(Math.floor(index/ASP)%REG); as.push(index%ASP); }
}
out += ia("RANK", rk); out += ia("REGION", rg); out += ia("ASPECT", as);
process.stdout.write(out);
