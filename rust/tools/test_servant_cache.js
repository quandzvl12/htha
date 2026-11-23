// Bước 3 — kiểm thử sprite cache Anh Linh bằng canvas THẬT (@napi-rs/canvas): so từng điểm ảnh với vẽ trực tiếp.
// Chạy: npm i --no-save @napi-rs/canvas && node rust/tools/test_servant_cache.js
const fs = require("fs");
const path = require("path");
const { createCanvas } = require("@napi-rs/canvas");
const L = fs.readFileSync(path.join(__dirname, "../../index.html"), "utf8").split("\n");
const a = L.findIndex((l) => l.includes("SERVANT MODELS")) - 1;
const j = L.findIndex((l, i) => i > a && /^  function drawFx\(\)/.test(l));
const c0 = L.findIndex((l) => l.includes("BƯỚC 3 — HIỆU NĂNG"));
const c1 = L.findIndex((l, i) => i > c0 && l.includes("SERVANT MODELS")) - 1;
if (a < 0 || j < 0 || c0 < 0) throw new Error("không tìm thấy mã Anh Linh / cache trong index.html");
const mk = new Function("TAU", "document", "localStorage", "canvas",
  "let ctx=null,gameTime=0,W=400,Q=2;\n" + L.slice(a, j).join("\n") + "\n" + L.slice(c0, c1).join("\n") +
  "\nreturn {setCtx:c=>{ctx=c},setT:v=>{gameTime=v},setQ:v=>{Q=v},drawArtoria,drawGilgamesh,drawEnkidu,drawServantModel};");
const SZ = 360, BG = [0x30, 0x30, 0x40];
let failed = 0;
const check = (ok, msg) => { console.log((ok ? "  ✓ " : "  ✗ ") + msg); if (!ok) failed++; };

function env(dev, ls = { getItem: () => null }) {
  const main = createCanvas(Math.round(SZ * dev), Math.round(SZ * dev));
  const M = mk(Math.PI * 2, { createElement: () => createCanvas(1, 1) }, ls, main);
  const img = (cb) => {
    const c = createCanvas(main.width, main.height), x = c.getContext("2d");
    x.fillStyle = "#303040"; x.fillRect(0, 0, c.width, c.height);
    M.setCtx(x); x.setTransform(dev, 0, 0, dev, 0, 0); x.save(); x.translate(SZ / 2, SZ / 2); cb(); x.restore();
    return x.getImageData(0, 0, c.width, c.height).data;
  };
  return { M, img };
}
function diff(A, B) {
  let sum = 0, big = 0, max = 0, cov = 0;
  for (let i = 0; i < A.length; i += 4) {
    const bgA = A[i] === BG[0] && A[i + 1] === BG[1] && A[i + 2] === BG[2], bgB = B[i] === BG[0] && B[i + 1] === BG[1] && B[i + 2] === BG[2];
    if (bgA && bgB) continue;
    const d = Math.max(Math.abs(A[i] - B[i]), Math.abs(A[i + 1] - B[i + 1]), Math.abs(A[i + 2] - B[i + 2]));
    cov++; sum += d; if (d > 24) big++; if (d > max) max = d;
  }
  return { cov, mean: sum / Math.max(1, cov), pctBig: 100 * big / Math.max(1, cov), max };
}
const mkS = (name, r, npT = 9) => ({ label: name, r, np: name === "Gilgamesh" ? "gate" : name === "Artoria" ? "excal" : "enuma", npT, x: 0, y: 0 });

console.log("1) Sprite cache = vẽ trực tiếp (cùng thời điểm, vị trí nguyên)");
for (const name of ["Artoria", "Gilgamesh", "Enkidu"]) for (const dev of [1, 1.5, 2]) for (const r of [14, 18, 20, 26]) {
  const { M, img } = env(dev), fn = M["draw" + name], s = mkS(name, r), t = 3.21;
  const direct = img(() => fn(s, t));
  const s2 = { ...s };
  img(() => { M.setT(t); M.drawServantModel(s2, t, fn); });
  const cached = img(() => { M.setT(t); M.drawServantModel(s2, t, fn); });
  const d = diff(direct, cached);
  check(d.mean < 2 && d.pctBig === 0 && d.max <= 24, `${name} dev=${dev} r=${r}: lệch TB ${d.mean.toFixed(2)}/255, >24: ${d.pctBig.toFixed(2)}%, max ${d.max}`);
}

console.log("1b) Quét nhiều pha hoạt họa (tư thế rộng nhất không được bị cắt khỏi sprite)");
for (const name of ["Artoria", "Gilgamesh", "Enkidu"]) {
  let worst = 0, worstBig = 0;
  for (let t = 0.1; t < 14; t += 0.37) for (const r of [14, 20, 26]) {
    const { M, img } = env(1), fn = M["draw" + name], s = mkS(name, r);
    const direct = img(() => fn(s, t));
    const cached = img(() => { M.setT(t); M.drawServantModel({ ...s }, t, fn); });
    const d = diff(direct, cached); worst = Math.max(worst, d.max); worstBig = Math.max(worstBig, d.pctBig);
  }
  /* vài điểm ảnh lẻ trên cạnh hình có thể lệch do khử răng cưa; cắt cụt sprite thì lệch hàng % điểm ảnh, tới ~200/255 */
  check(worstBig <= 0.1 && worst <= 64, `${name}: 38 pha × 3 cỡ — lệch lớn nhất ${worst}/255, điểm ảnh lệch >24: ${worstBig.toFixed(2)}%`);
}

console.log("2) Độ trễ hoạt họa 66 ms (làm mới 15 Hz) không lớn hơn mức đổi tự nhiên giữa các khung");
for (const name of ["Artoria", "Gilgamesh", "Enkidu"]) {
  const { M, img } = env(1), fn = M["draw" + name], s = mkS(name, 18), t0 = 3.21;
  const s2 = { ...s };
  img(() => { M.setT(t0); M.drawServantModel(s2, t0, fn); });                 // làm mới tại t0
  const cachedLate = img(() => { M.setT(t0 + .06); M.drawServantModel(s2, t0 + .06, fn); }); // chưa tới hạn làm mới (<1/15 s)
  const directLate = img(() => fn(s, t0 + .06));
  const d = diff(directLate, cachedLate);
  check(d.mean < 6, `${name}: trễ ≈60 ms → lệch TB ${d.mean.toFixed(2)}/255`);
}

console.log("3) Làm mới / vô hiệu hóa đúng lúc");
{
  const { M, img } = env(1), fn = M.drawGilgamesh, s = mkS("Gilgamesh", 18, 9);
  img(() => { M.setT(1); M.drawServantModel(s, 1, fn); });
  const sp1 = s._sv, t1 = sp1.t0;
  img(() => { M.setT(1.02); M.drawServantModel(s, 1.02, fn); });
  check(s._sv.t0 === t1, "trong 1/15 s: dùng lại sprite (không vẽ lại)");
  img(() => { M.setT(1.2); M.drawServantModel(s, 1.2, fn); });
  check(s._sv.t0 > t1, "sau 1/15 s: làm mới sprite");
  s.npT = .2;
  img(() => { M.setT(1.21); M.drawServantModel(s, 1.21, fn); });
  check(s._sv.fl === 1, "bắt đầu tia gate (npT<.4): sprite được tạo lại");
  s.r = 26;
  img(() => { M.setT(1.22); M.drawServantModel(s, 1.22, fn); });
  check(s._sv.pw > sp1.pw, "đổi bán kính: kích thước sprite đổi theo");
  img(() => { M.setT(0.5); M.drawServantModel(s, 0.5, fn); });
  check(s._sv.t0 <= 0.5, "gameTime lùi (ván mới): làm mới ngay");
}
{
  const { M, img } = env(1, { getItem: (k) => (k === "htahNoSvCache" ? "1" : null) });
  const s = mkS("Artoria", 18);
  img(() => { M.setT(1); M.drawServantModel(s, 1, M.drawArtoria); });
  check(!s._sv, "htahNoSvCache=1: tắt cache, vẽ trực tiếp");
}
{
  const { M } = env(1), s = mkS("Enkidu", 18);
  const c = createCanvas(50, 50), x = c.getContext("2d"); x.rotate(.3); M.setCtx(x);
  M.drawServantModel(s, 1, M.drawEnkidu);
  check(!s._sv, "canvas đang xoay: vẽ trực tiếp, không dùng cache");
}
console.log(failed ? `\n${failed} kiểm tra THẤT BẠI` : "\nTất cả kiểm tra sprite cache đều đạt");
process.exit(failed ? 1 : 0);
