// Kiểm tra mọi hàm WASM cần nhập đều có trong trình tải macroquad (mq_js_bundle.js).
// Dùng: node rust/tools/check_wasm_imports.js <file.wasm> <mq_js_bundle.js>
const fs = require("fs");
const [wasm, bundle] = process.argv.slice(2);
const m = new WebAssembly.Module(fs.readFileSync(wasm));
const js = fs.readFileSync(bundle, "utf8");
const imps = WebAssembly.Module.imports(m);
const missing = imps.filter((i) => !js.includes(i.name));
const exps = WebAssembly.Module.exports(m).map((e) => e.name);
console.log(`wasm nhập ${imps.length} hàm; thiếu trong trình tải: ${missing.length}`);
for (const need of ["main", "frame", "memory"]) if (!exps.includes(need)) { console.error("thiếu export", need); process.exit(1); }
if (missing.length) { console.error("THIẾU:", missing.map((i) => i.name).join(", ")); process.exit(1); }
