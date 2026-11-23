#!/usr/bin/env python3
"""Bước 3 (hiệu năng): sprite cache cho Anh Linh. Cần apply_step1/2 trước. Idempotent."""
import pathlib, sys
root = pathlib.Path(__file__).resolve().parents[2]
p = root / "index.html"
s = p.read_text(encoding="utf8")
if "function drawServantModel(" in s:
    print("Bước 3 đã vá — bỏ qua"); sys.exit(0)
cache = (root / "rust/web/servant_cache.js").read_text(encoding="utf8")

old = '''      if(s.label==="Artoria")drawArtoria(s,t);
      else if(s.label==="Gilgamesh")drawGilgamesh(s,t);
      else if(s.label==="Enkidu")drawEnkidu(s,t);'''
if s.count(old) != 1:
    sys.exit(f"LỖI: mong đợi 1 chỗ gọi mô hình Anh Linh, thấy {s.count(old)}")
s = s.replace(old, '''      if(s.label==="Artoria")drawServantModel(s,t,drawArtoria);
      else if(s.label==="Gilgamesh")drawServantModel(s,t,drawGilgamesh);
      else if(s.label==="Enkidu")drawServantModel(s,t,drawEnkidu);''')

anchor = "  /* ======================================================\n     SERVANT MODELS — Artoria / Gilgamesh / Enkidu"
if s.count(anchor) != 1:
    sys.exit(f"LỖI: mong đợi 1 mốc SERVANT MODELS, thấy {s.count(anchor)}")
s = s.replace(anchor, cache + anchor)
p.write_text(s, encoding="utf8")
print("Đã vá index.html (bước 3): sprite cache Anh Linh")
