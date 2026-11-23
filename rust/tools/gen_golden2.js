// Bước 2 — sinh dữ liệu đối chiếu bằng cách CHẠY CHÍNH các hàm gốc trích từ index.html:
//   profile:  profileLevelInfo / cleanProfileName / normalizePlayerProfile
//   saves:    writeClassSave / deleteClassSave (nhánh khách VÀ nhánh tài khoản) với localStorage giả
// Chạy: node rust/tools/gen_golden2.js   (ghi rust/htha-core/tests/golden_profile.json, golden_saves.json)
const fs = require("fs");
const path = require("path");
const html = fs.readFileSync(path.join(__dirname, "../../index.html"), "utf8");
const lines = html.split("\n");
const idx = (re) => { const i = lines.findIndex((l) => re.test(l)); if (i < 0) throw new Error("không thấy " + re); return i; };
const slice = (a, b) => lines.slice(idx(a), idx(b)).join("\n");

// Core ở chế độ JS thuần: bản đã vá gọi Core.* nhưng khi chưa có WASM sẽ rơi về code JS gốc (…JS)
const bridgeM = html.match(/<script>(\/\* ===== HTHA CORE[\s\S]*?)<\/script>/);
const Core = bridgeM ? (() => { const f = globalThis.fetch; globalThis.fetch = undefined;
  try { return new Function(bridgeM[1] + ";return Core;")(); } finally { globalThis.fetch = f; } })() : {};

// ---------- PRNG xác định ----------
let seed = 20261122;
const rnd = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };
const pick = (a) => a[Math.floor(rnd() * a.length)];

// ---------- PROFILE ----------
const CLASS_KEYS = ["warrior", "archer", "mage", "hero", "summoner", "hakaishin"];
const NOW = 1700000000000;
const profileSrc = slice(/function profileXPForNextLevel/, /function profileDisplayName/);
const avatarsLine = lines[idx(/const PROFILE_AVATARS=/)];
const profileEnv = new Function("classes", "Date", "Core",
  "const MAX_DUNGEONS=300,STAGES_PER_DUNGEON=10,MAX_STAGES=3000;" + avatarsLine + "\n" + profileSrc +
  "\nreturn {profileLevelInfo,cleanProfileName,normalizePlayerProfile};");
const classesObj = Object.fromEntries(CLASS_KEYS.map((k) => [k, {}]));
const env = profileEnv(classesObj, { now: () => NOW }, Core);

const pool = [null, 0, 1, 2, -5, 3.7, 1e6, 99999999999, "12", "abc", "", " 7 ", "0x10", "1e3", "5.", true, false, [], [5], [1, 2], {}, { a: 1 }, "Kiếm Sĩ", "  nhiều   khoảng   trắng  ", "<b>x</b>\u0007y", "A".repeat(50), "\u00a0\u3000", "🛡️ Tên emoji 🐉", "Kết thúc"];
const maybe = (o, k, v) => { if (v !== undefined) o[k] = v; };
const pv = () => (rnd() < 0.12 ? undefined : pick(pool));
const histItem = () => {
  if (rnd() < 0.1) return pick([null, 5, "x", [], [1, 2]]);
  const it = {};
  for (const k of ["at", "stage", "dungeon", "level", "kills", "duration", "xpGained", "reason", "mode"]) maybe(it, k, pv());
  maybe(it, "classKey", rnd() < 0.5 ? pick(CLASS_KEYS) : rnd() < 0.5 ? pick(["bogus", "", 5, null]) : undefined);
  return it;
};
const rawProfile = () => {
  if (rnd() < 0.06) return pick([null, 5, "str", [], [1], true]);
  const r = {};
  for (const k of ["name", "totalXp", "totalRuns", "bestStage", "totalKills", "playSeconds"]) maybe(r, k, pv());
  maybe(r, "avatar", rnd() < 0.5 ? pick(["🧑‍🚀", "🛡️", "🏹", "🦊", "🌟"]) : pick(["x", "", 3, null, undefined]));
  const hr = rnd();
  if (hr < 0.6) r.history = Array.from({ length: Math.floor(rnd() * 26) }, histItem);
  else if (hr < 0.8) r.history = pick([null, "x", {}, 5]);
  return r;
};
const fixed = [
  {}, { totalXp: 500 }, { totalXp: 120 + 30 * 1 }, { totalXp: 1e9 }, { totalXp: "150" }, { totalXp: -40 },
  { name: "  Quân   Đẹp\u0001 trai <3  ", avatar: "🐉", totalXp: 4321, totalRuns: 7, bestStage: 250, totalKills: 9001, playSeconds: 3600,
    history: [{ at: 1690000000000, classKey: "mage", stage: 120, level: 33, kills: 500, duration: 640, xpGained: 880, reason: "Bị hạ gục", mode: "Thử thách" }] },
  { name: "", totalRuns: null }, { name: "   ", totalRuns: "abc" }, { name: "\u00a0" }, { name: 12345 },
  { history: Array.from({ length: 30 }, (_, i) => ({ stage: i + 1, classKey: "archer" })) },
];
const profileCases = [];
for (let i = 0; i < fixed.length + 600; i++) {
  const raw = i < fixed.length ? fixed[i] : rawProfile();
  const fb = pick(["Người chơi", "quan", "", "Tên rất rất rất rất dài hơn hai mươi", "tài khoản 01"]);
  const rawJson = JSON.stringify(raw === undefined ? null : raw);
  const copy = JSON.parse(rawJson);
  profileCases.push({ raw: copy, fallback: fb, expected: JSON.parse(JSON.stringify(env.normalizePlayerProfile(copy, fb))) });
}
const levelCases = [];
for (const xp of [0, 1, 149, 150, 151, 329, 330, 500, 1000, 5000, 12345, 99999, 400000, 1e6, 1e7, 123456789]) levelCases.push([xp, env.profileLevelInfo(xp)]);
fs.writeFileSync(path.join(__dirname, "../htha-core/tests/golden_profile.json"),
  JSON.stringify({ now: NOW, classKeys: CLASS_KEYS, cases: profileCases, levels: levelCases }));

// ---------- SAVES ----------
const accKeyLine = lines[idx(/const ACCOUNTS_KEY=/)];
const guestKeyLine = lines[idx(/const GUEST_SAVES_KEY=/)];
const loadAcc = lines[idx(/function loadAccounts\(\)/)], saveAcc = lines[idx(/function saveAccounts\(/)];
const savesSrc = slice(/function _guestMap\(\)/, /function enterClassScreen/);
const makeEnv = () => {
  const store = {};
  const localStorage = { getItem: (k) => (k in store ? store[k] : null), setItem: (k, v) => { store[k] = String(v); }, removeItem: (k) => { delete store[k]; } };
  const f = new Function("localStorage", "showToast", "console", "Core",
    accKeyLine + "\n" + guestKeyLine + "\nlet currentAccount=null, save=null;\n" + loadAcc + "\n" + saveAcc + "\n" + savesSrc +
    "\nreturn {writeClassSave,deleteClassSave,setAcc:v=>{currentAccount=v},LS:localStorage};");
  return { store, api: f(localStorage, () => {}, console, Core) };
};
const snapshot = (mode, store) => {
  if (mode === "guest") return {
    last: JSON.parse(store["hanhTrinhAnhHungSave"] || "null"),
    saves: JSON.parse(store["hanhTrinhAnhHungSaves"] || "{}") };
  const rec = JSON.parse(store["hanhTrinhAccounts"] || "{}").u || {};
  return { last: rec.save || null, saves: rec.saves || {} };
};
const saveCases = [];
for (let n = 0; n < 220; n++) {
  const mode = n % 2 ? "account" : "guest";
  const { store, api } = makeEnv();
  if (mode === "account") { store["hanhTrinhAccounts"] = JSON.stringify({ u: { password: "" } }); api.setAcc("u"); }
  const ops = [];
  const nops = 4 + Math.floor(rnd() * 12);
  for (let k = 0; k < nops; k++) {
    const key = pick(CLASS_KEYS.slice(0, 5));
    if (rnd() < 0.7) {
      const data = { classKey: key, savedAt: rnd() < 0.3 ? pick([0, 1000, 1000, 2000]) : Math.floor(rnd() * 5000), stage: Math.floor(rnd() * 99), n: k };
      if (rnd() < 0.08) delete data.savedAt;
      api.writeClassSave(JSON.parse(JSON.stringify(data)));
      ops.push({ op: "write", data, after: snapshot(mode, store) });
    } else {
      api.deleteClassSave(key);
      ops.push({ op: "delete", key, after: snapshot(mode, store) });
    }
  }
  saveCases.push({ mode, ops });
}
fs.writeFileSync(path.join(__dirname, "../htha-core/tests/golden_saves.json"), JSON.stringify(saveCases));
console.log("profile:", profileCases.length, "ca |", "levels:", levelCases.length, "| saves:", saveCases.length, "chuỗi,",
  saveCases.reduce((a, c) => a + c.ops.length, 0), "thao tác");
