/* ===== HTHA CORE — lõi Rust/WASM (BƯỚC 1: công thức cân bằng) =====
   Game luôn chạy được: ban đầu dùng bản JS dự phòng; khi htha_core.wasm tải xong VÀ
   khớp bản JS trên các giá trị mẫu thì tự chuyển sang Rust. Lệch → giữ JS và báo ở console. */
const Core=(()=>{
  const js={
    enemyHpMult:s=>Math.max(1,0.1244*Math.pow(s,1.2144)),
    enemyDmgMult:s=>Math.max(1,0.2093*Math.pow(s,1.4806)),
    bossHpMult:(s,b)=>b?Math.max(.0001,7.75*Math.pow(s,-0.811)):1,
    bossDmgMult:(s,b)=>b?Math.max(.0001,4.199*Math.pow(s,-0.364)):1,
    goldPerKill:s=>30*(1+s*0.005),
    sellValue:(s,m,e)=>Math.round(300*(1+s*0.02)*Math.pow(m,0.9)*(1+e*0.25)),
    upgradeCost:(k,lv)=>Math.max(1,Math.round(20*Math.pow(1.2,lv))),
    skillValue:(b,t)=>Math.max(1,Math.round(b*t*0.6)),
    dungeonOfStage:s=>Math.min(300,Math.max(1,Math.ceil(s/10))),
    floorOfStage:s=>(s-1)%10+1,
    rankIndex:d=>d===1?0:d<=20?1:d<=60?2:d<=120?3:d<=180?4:d<=240?5:d<=280?6:7,
    regionIndex:(d,rl,al)=>d===1?-1:Math.floor((d-2)/al)%rl,
    aspectIndex:(d,al)=>d===1?-1:(d-2)%al
  };
  const KIND={power:0,speed:1,health:2};
  const api=Object.assign({backend:"js"},js);
  function wrap(x){
    return {
      enemyHpMult:s=>x.enemy_hp_mult(s),
      enemyDmgMult:s=>x.enemy_dmg_mult(s),
      bossHpMult:(s,b)=>x.boss_hp_mult(s,b?1:0),
      bossDmgMult:(s,b)=>x.boss_dmg_mult(s,b?1:0),
      goldPerKill:s=>x.gold_per_kill(s),
      sellValue:(s,m,e)=>x.sell_value(s,m,e),
      upgradeCost:(k,lv)=>x.upgrade_cost(typeof k==="number"?k:(KIND[k]|0),lv),
      skillValue:(b,t)=>x.skill_value(b,t),
      dungeonOfStage:s=>x.dungeon_of_stage(s),
      floorOfStage:s=>x.floor_of_stage(s),
      rankIndex:d=>x.rank_index(d),
      regionIndex:(d,rl,al)=>x.region_index(d,rl,al),
      aspectIndex:(d,al)=>x.aspect_index(d,al)
    };
  }
  function same(a,b){return a===b||Math.abs(a-b)<=1e-9*Math.max(1,Math.abs(b))}
  function parity(w){
    const S=[1,2,7,10,11,55,100,101,333,1000,2500,3000],R=[1,2.4,5,10,20];
    for(const s of S){
      if(!same(w.enemyHpMult(s),js.enemyHpMult(s))||!same(w.enemyDmgMult(s),js.enemyDmgMult(s))||
         !same(w.bossHpMult(s,true),js.bossHpMult(s,true))||!same(w.bossDmgMult(s,true),js.bossDmgMult(s,true))||
         !same(w.goldPerKill(s),js.goldPerKill(s))||w.dungeonOfStage(s)!==js.dungeonOfStage(s)||
         w.floorOfStage(s)!==js.floorOfStage(s))return false;
      for(const r of R)for(const e of [0,2])if(w.sellValue(s,r,e)!==js.sellValue(s,r,e))return false;
    }
    for(let lv=0;lv<60;lv+=7)if(w.upgradeCost("power",lv)!==js.upgradeCost("power",lv))return false;
    for(const b of [3,10,25.5,100])for(const t of [1,1.15,2.35])if(w.skillValue(b,t)!==js.skillValue(b,t))return false;
    for(let d=1;d<=300;d+=13)
      if(w.rankIndex(d)!==js.rankIndex(d)||w.regionIndex(d,20,16)!==js.regionIndex(d,20,16)||w.aspectIndex(d,16)!==js.aspectIndex(d,16))return false;
    return true;
  }
  try{
    if(typeof WebAssembly==="object"&&typeof fetch==="function"){
      fetch("htha_core.wasm").then(r=>{if(!r.ok)throw new Error("HTTP "+r.status);return r.arrayBuffer()})
        .then(buf=>WebAssembly.instantiate(buf,{}))
        .then(({instance})=>{
          const w=wrap(instance.exports);
          if(parity(w)){Object.assign(api,w);api.backend="rust-wasm";console.info("[HTHA] lõi Rust/WASM đang chạy")}
          else console.warn("[HTHA] WASM lệch bản JS — giữ JS")
        }).catch(e=>console.info("[HTHA] dùng bản JS (không có WASM):",e&&e.message))
    }
  }catch(e){}
  return api;
})();
