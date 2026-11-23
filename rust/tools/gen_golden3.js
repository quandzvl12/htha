// Bước 4a — dữ liệu đối chiếu chạy CHÍNH các hàm gốc trong game (qua jsdom):
//   spawnEnemy (chỉ số quái, với dòng Math.random cố định), applyClass (chỉ số người chơi), gainXP (XP/lên cấp).
// Chạy: npm i --no-save jsdom@24 && node rust/tools/gen_golden3.js  → rust/htha-core/tests/golden_combat.json
const fs = require("fs"), path = require("path");
const { JSDOM, VirtualConsole } = require("jsdom");
let html = fs.readFileSync(path.join(__dirname, "../../index.html"), "utf8");
const MARK = "\n  requestAnimationFrame(loop);\n";
if (html.split(MARK).length !== 2) throw new Error("mốc requestAnimationFrame(loop) không duy nhất");
html = html.replace(MARK, `\n  window.__g={setClass:k=>{selectedClass=k},applyClass,setStage:v=>{stage=v},
    setUp:(p,s,h)=>{power=p;speedLevel=s;healthLevel=h},spawn:(b,i,f)=>{enemies=[];spawnEnemy(b,i,f);return enemies[0]},
    player:()=>player,view:()=>({W,H}),setXp:(l,x)=>{level=l;xp=x;xpNeed=l<=1?80:Math.floor(80+l*24)},
    gainXP:n=>{try{gainXP(n)}catch(e){}},xpState:()=>({level,xp,xpNeed}),resetRun,startStage,setGameUI,
    unpause:()=>{running=true;paused=false;choosing=false;document.getElementById("level-modal")?.classList.add("hidden")}};\n  requestAnimationFrame(loop);\n`);
const noop = () => {}, grad = { addColorStop: noop };
const ctx = (canvas) => new Proxy({ canvas, measureText: () => ({ width: 10 }), getImageData: () => ({ data: new Uint8ClampedArray(4) }),
  createPattern: () => ({}), createRadialGradient: () => grad, createLinearGradient: () => grad, getTransform: () => ({ a: 1, b: 0, c: 0, d: 1, e: 0, f: 0 }) },
  { get: (t, p) => (p in t ? t[p] : typeof p === "symbol" ? undefined : noop), set: (t, p, v) => { t[p] = v; return true } });
const vc = new VirtualConsole();
const dom = new JSDOM(html, { runScripts: "dangerously", pretendToBeVisual: true, url: "https://localhost/", virtualConsole: vc,
  beforeParse(w) {
    w.HTMLCanvasElement.prototype.getContext = function () { return this.__c || (this.__c = ctx(this)) };
    w.matchMedia = () => ({ matches: false, addEventListener: noop, removeEventListener: noop, addListener: noop });
    w.ResizeObserver = class { observe() {} disconnect() {} };
    w.requestAnimationFrame = () => 1; w.cancelAnimationFrame = noop;
    w.AudioContext = w.webkitAudioContext = class { constructor() { this.destination = {}; this.currentTime = 0 } createGain() { return { gain: { value: 1, setValueAtTime: noop, linearRampToValueAtTime: noop, exponentialRampToValueAtTime: noop }, connect: noop } }
      createOscillator() { return { frequency: { value: 0, setValueAtTime: noop, exponentialRampToValueAtTime: noop }, connect: noop, start: noop, stop: noop } } resume() { return Promise.resolve() } };
    w.fetch = () => Promise.reject(new Error("offline"));
    w.HTMLMediaElement.prototype.play = () => Promise.resolve(); w.HTMLMediaElement.prototype.pause = noop;
  } });
const w = dom.window;
setTimeout(() => {
  const g = w.__g; if (!g) { console.error("không gắn được hook"); process.exit(2) }
  const out = { spawn: [], player: [], xp: [] };
  // PRNG cố định cho Math.random của game
  let seed = 424242; const next = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296 };
  let forced = null, log = [];
  w.Math.random = () => { const v = forced && log.length < forced.length ? forced[log.length] : next(); log.push(v); return v };
  const stages = [1, 2, 5, 9, 10, 11, 19, 20, 37, 50, 99, 100, 101, 150, 250, 333, 500, 777, 1000, 1500, 2500, 3000];
  g.setClass("archer"); g.setUp(0, 0, 0);
  const inf = (x) => (Number.isFinite(x) ? x : null);
  for (const st of stages) {
    g.setStage(st); g.applyClass();
    const variants = [];
    for (let i = 0; i < 8; i++) variants.push({ boss: false, index: i, forced: null });
    variants.push({ boss: true, index: 25, forced: null }, { boss: false, index: 26, forced: "tayhung" }, { boss: false, index: 26, forced: "thanh" });
    for (const v of variants) for (const eliteU of v.boss || v.forced ? [0.5] : [0.0005, 0.99]) {
      const rolls = [next(), next(), next(), eliteU];
      forced = rolls; log = [];
      const p = g.player(), view = g.view(), px = p.x, py = p.y;
      const e = g.spawn(v.boss, v.index, v.forced);
      forced = null;
      out.spawn.push({ stage: st, boss: v.boss, index: v.index, forced: v.forced, rolls, px, py, W: view.W, H: view.H,
        exp: { kind: e.type, name: e.name, hp: e.hp, maxHp: e.maxHp, damage: e.damage, rate: e.rate, r: e.r, speed: e.speed, isBoss: !!e.isBoss, hiddenBoss: !!e.hiddenBoss,
          bossTier: e.bossTier, damageTakenMult: e.damageTakenMult, damageOutMult: e.damageOutMult, freezeResist: !!e.freezeResist, reflect: !!e.reflect,
          lifesteal: e.lifesteal, elite: !!e.elite, x: e.x, y: e.y, lastAttack: e.lastAttack, skillTimer: inf(e.skillTimer), teleportTimer: inf(e.teleportTimer),
          timeStopTimer: inf(e.timeStopTimer), blackHoleTimer: inf(e.blackHoleTimer) } });
    }
  }
  // chỉ số người chơi (applyClass)
  const classKeys = ["warrior", "archer", "mage", "hero", "summoner", "goon", "slime", "hakaishin"];
  for (const k of classKeys) for (const st of [1, 2, 10, 50, 100, 500, 1000, 3000]) for (const up of [[0, 0, 0], [1, 0, 0], [5, 3, 2], [20, 12, 10], [0, 40, 0], [60, 29, 31]]) {
    g.setClass(k); g.setStage(st); g.setUp(...up); g.applyClass();
    const p = g.player();
    out.player.push({ cls: k, stage: st, up, exp: { maxHp: p.maxHp, speed: p.speed, damage: p.damage, range: p.range, rate: p.rate } });
  }
  // XP / lên cấp
  const cases = [];
  for (const L of [1, 2, 5, 50, 300, 790, 799, 800]) for (const frac of [0, 0.5, 0.999]) for (const n of [1, 18, 120, 5000, 1e6, 1e9]) cases.push([L, frac, n]);
  for (const [L, frac, n] of cases) {
    const need = L <= 1 ? 80 : Math.floor(80 + L * 24), x0 = Math.floor(need * frac);
    g.unpause(); g.setXp(L, x0); g.gainXP(n);
    const s = g.xpState();
    out.xp.push({ level: L, xp: x0, gain: n, exp: { level: s.level, xp: s.xp, need: s.xpNeed } });
  }
  fs.writeFileSync(path.join(__dirname, "../htha-core/tests/golden_combat.json"), JSON.stringify(out));
  console.log("spawn:", out.spawn.length, "| player:", out.player.length, "| xp:", out.xp.length);
  process.exit(0);
}, 1500);
