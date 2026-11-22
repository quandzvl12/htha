#!/usr/bin/env python3
"""Bước 1: vá index.html để gọi lõi Rust (Core.*) thay cho công thức JS. Chạy được lặp lại (idempotent)."""
import pathlib, sys
root = pathlib.Path(__file__).resolve().parents[2]
p = root / "index.html"
s = p.read_text(encoding="utf8")
if "const Core=(()=>" in s:
    print("index.html đã vá bước 1 rồi — bỏ qua"); sys.exit(0)

bridge = (root / "rust/web/core_bridge.js").read_text(encoding="utf8")

# (cũ, mới, số lần xuất hiện mong đợi)
EDITS = [
 ("Math.max(5,Math.round(BAL.sellBase*(1+stage*BAL.sellStageK)*Math.pow(r.mult,BAL.sellExp)))",
  "Math.max(5,Core.sellValue(stage,r.mult,0))", 2),
 ("base=Math.max(base,Math.round(BAL.sellBase*stageFactor*Math.pow(rarity.mult,BAL.sellExp)*enhanceFactor));",
  "base=Math.max(base,Core.sellValue(Math.max(0,Number(stage)||1),rarity.mult,Math.max(0,Number(it.enhance)||0)));", 1),
 ("return Math.max(1,Math.round(UPGRADE_BASE[kind]*Math.pow(1+UP_COST_PCT,lv)))",
  "return Core.upgradeCost(kind,lv)", 1),
 ("const _hm=Math.max(1,BAL.enemyHpA*Math.pow(stage,BAL.enemyHpB)),_dm=Math.max(1,BAL.enemyDmgA*Math.pow(stage,BAL.enemyDmgB));",
  "const _hm=Core.enemyHpMult(stage),_dm=Core.enemyDmgMult(stage);", 1),
 ("const _bh=boss?Math.max(.0001,BAL.bossHpA*Math.pow(stage,BAL.bossHpB)):1,_bd=boss?Math.max(.0001,BAL.bossDmgA*Math.pow(stage,BAL.bossDmgB)):1;",
  "const _bh=Core.bossHpMult(stage,!!boss),_bd=Core.bossDmgMult(stage,!!boss);", 1),
 ("Math.round(BAL.goldPerKill*(1+stage*BAL.goldStageK)*(player.mods?.gold||1))",
  "Math.round(Core.goldPerKill(stage)*(player.mods?.gold||1))", 1),
 ("p=b=>Math.max(1,Math.round(b*t*BAL.skillScale))",
  "p=b=>Core.skillValue(b,t)", 1),
 ('function dungeonRank(number){if(number===1)return"Tân Thủ";if(number<=20)return"Đồng";if(number<=60)return"Bạc";if(number<=120)return"Vàng";if(number<=180)return"Kim Cương";if(number<=240)return"Huyền Thoại";if(number<=280)return"Thần Thoại";return"Huỷ Diệt"}',
  'const DUNGEON_RANKS=["Tân Thủ","Đồng","Bạc","Vàng","Kim Cương","Huyền Thoại","Thần Thoại","Huỷ Diệt"];function dungeonRank(number){return DUNGEON_RANKS[Core.rankIndex(number)]}', 1),
 ('const index=number-2;return"Dungeon "+DUNGEON_REGIONS[Math.floor(index/DUNGEON_ASPECTS.length)%DUNGEON_REGIONS.length]+" "+DUNGEON_ASPECTS[index%DUNGEON_ASPECTS.length]}',
  'return"Dungeon "+DUNGEON_REGIONS[Core.regionIndex(number,DUNGEON_REGIONS.length,DUNGEON_ASPECTS.length)]+" "+DUNGEON_ASPECTS[Core.aspectIndex(number,DUNGEON_ASPECTS.length)]}', 1),
 ("const dungeon=Math.min(MAX_DUNGEONS,Math.max(1,Math.ceil(stage/STAGES_PER_DUNGEON))),floor=(stage-1)%STAGES_PER_DUNGEON+1;return dungeonName(dungeon)",
  "const dungeon=Core.dungeonOfStage(stage),floor=Core.floorOfStage(stage);return dungeonName(dungeon)", 1),
]
for old, new, n in EDITS:
    c = s.count(old)
    if c != n:
        sys.exit(f"LỖI: mong đợi {n} chỗ nhưng thấy {c}: {old[:70]}...")
    s = s.replace(old, new)

i = s.index("<script>")
s = s[:i] + "<script>" + bridge + "</script>\n" + s[i:]
p.write_text(s, encoding="utf8")
print("Đã vá index.html (bước 1):", len(EDITS), "nhóm thay thế + cầu nối Core")
