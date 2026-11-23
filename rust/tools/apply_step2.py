#!/usr/bin/env python3
"""Bước 2: nâng cầu nối Core lên bản 2 và vá index.html dùng lõi Rust cho hồ sơ + save theo nghề.
Cần chạy apply_step1.py trước. Idempotent."""
import pathlib, re, sys
root = pathlib.Path(__file__).resolve().parents[2]
p = root / "index.html"
s = p.read_text(encoding="utf8")
if "const Core=(()=>" not in s:
    sys.exit("Chưa vá bước 1 — chạy rust/tools/apply_step1.py trước")
bridge = (root / "rust/web/core_bridge.js").read_text(encoding="utf8")

# 1) thay khối cầu nối cũ bằng bản mới (luôn làm, để cập nhật bridge)
a = s.index("<script>/* ===== HTHA CORE")
b = s.index("</script>", a)
s = s[:a] + "<script>" + bridge + s[b:]

if "normalizePlayerProfileJS" in s:
    p.write_text(s, encoding="utf8"); print("Bước 2 đã vá — chỉ cập nhật cầu nối"); sys.exit(0)

def must(old, new, n=1):
    global s
    c = s.count(old)
    if c != n:
        sys.exit(f"LỖI: mong đợi {n} chỗ nhưng thấy {c}: {old[:80]}...")
    s = s.replace(old, new)

must("function profileXPForNextLevel(level){return 120+level*30}",
     "function profileXPForNextLevel(level){return Core.profileXpForNext(level)}")
must("""  function profileLevelInfo(totalXp){
    let remaining=Math.max(0,Math.floor(Number(totalXp)||0)),level=1,need=profileXPForNextLevel(level);
    while(level<100&&remaining>=need){remaining-=need;level++;need=profileXPForNextLevel(level)}
    return{level,progress:level>=100?0:remaining,need:level>=100?0:need}
  }""", "  function profileLevelInfo(totalXp){return Core.profileLevelInfo(totalXp)}")

must("  function normalizePlayerProfile(raw,fallbackName){",
     """  /* BƯỚC 2: chuẩn hóa hồ sơ bằng Rust; chưa có WASM / lỗi → dùng bản JS gốc bên dưới */
  function normalizePlayerProfile(raw,fallbackName){
    const r=Core.normalizeProfile(raw,fallbackName,Object.keys(classes),Date.now());
    return r||normalizePlayerProfileJS(raw,fallbackName)
  }
  function normalizePlayerProfileJS(raw,fallbackName){""")

must("  function writeClassSave(data){", "  function writeClassSaveJS(data){")
must("  function deleteClassSave(k){", "  function deleteClassSaveJS(k){")
must("  function reloadSaveFromStorage(){",
     """  /* BƯỚC 2: logic ghi/xóa save theo nghề chạy bằng Rust; JS chỉ đọc/ghi localStorage. Lỗi → bản JS gốc (…JS). */
  function writeClassSave(data){
    if(!Core.canSaves())return writeClassSaveJS(data);
    if(currentAccount){
      const ac=loadAccounts(),rec=ac[currentAccount]||(ac[currentAccount]={password:""});
      const r=Core.classSaveWrite({last:rec.save||null,saves:rec.saves||{}},data);
      if(!r)return writeClassSaveJS(data);
      rec.save=r.last;rec.saves=r.saves;saveAccounts(ac);
    }else{
      const r=Core.classSaveWrite({last:_guestLast(),saves:_guestMap()},data);
      if(!r)return writeClassSaveJS(data);
      localStorage.setItem(GUEST_SAVES_KEY,JSON.stringify(r.saves));
      localStorage.setItem("hanhTrinhAnhHungSave",JSON.stringify(r.last));
    }
  }
  function deleteClassSave(k){
    if(!Core.canSaves())return deleteClassSaveJS(k);
    if(currentAccount){
      const ac=loadAccounts(),rec=ac[currentAccount];if(!rec)return;
      const r=Core.classSaveDelete({last:rec.save||null,saves:rec.saves||{}},k);
      if(!r)return deleteClassSaveJS(k);
      rec.save=r.last;rec.saves=r.saves;saveAccounts(ac);
    }else{
      const r=Core.classSaveDelete({last:_guestLast(),saves:_guestMap()},k);
      if(!r)return deleteClassSaveJS(k);
      localStorage.setItem(GUEST_SAVES_KEY,JSON.stringify(r.saves));
      if(r.last)localStorage.setItem("hanhTrinhAnhHungSave",JSON.stringify(r.last));else localStorage.removeItem("hanhTrinhAnhHungSave");
    }
    reloadSaveFromStorage();
  }
  function reloadSaveFromStorage(){""")
p.write_text(s, encoding="utf8")
print("Đã vá index.html (bước 2): hồ sơ + save theo nghề → Rust, giữ bản JS gốc làm dự phòng")
