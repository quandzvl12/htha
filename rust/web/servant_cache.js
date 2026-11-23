  /* ===== BƯỚC 3 — HIỆU NĂNG: bộ nhớ đệm sprite cho Anh Linh (Artoria / Gilgamesh / Enkidu) =====
     Đo thực tế: mỗi Anh Linh tốn 2.000–6.000 lệnh canvas MỖI KHUNG (kèm hàng trăm lần bật shadowBlur),
     kể cả ở Q0. Mô hình chỉ phụ thuộc (nhãn, r, t, tia gate) nên vẽ vào canvas phụ ~SV_HZ lần/giây,
     các khung còn lại chỉ drawImage. Các Anh Linh được so le thời điểm làm mới để không dồn tải vào một khung.
     Sprite có gốc là số nguyên điểm ảnh thiết bị và được căn vào lưới điểm ảnh khi vẽ → nét không bị mờ.
     Trạng thái hiển thị duy nhất phụ thuộc chiêu cuối là tia của Gilgamesh (np="gate" và npT<.4) → nằm trong chữ ký.
     Tắt bằng localStorage htahNoSvCache=1. */
  const SV_HZ=15,SV_L=-72,SV_T=-84,SV_W=144,SV_H=138;   /* vùng vẽ đo được (r=20): x −57…56, y −70…39, cộng lề cho viền sáng/bóng mờ */
  let _svOff=false;try{_svOff=localStorage.getItem("htahNoSvCache")==="1"}catch(_){}
  function _svGate(c){try{const sd=Object.getOwnPropertyDescriptor(CanvasRenderingContext2D.prototype,"shadowBlur");
    Object.defineProperty(c,"shadowBlur",{get(){return sd.get.call(c)},set(v){sd.set.call(c,Q>=2?v:0)},configurable:true})}catch(_){}}
  function drawServantModel(s,t,fn){
    if(_svOff||typeof ctx.getTransform!=="function"){fn(s,t);return}
    const m=ctx.getTransform();
    if(m.b||m.c||!(m.a>0)||Math.abs(m.a-m.d)>1e-6){fn(s,t);return}      /* có xoay/méo/không đều → vẽ trực tiếp như cũ */
    const dev=Math.round(m.a*1000)/1000,k=s.r/20,
      ox=Math.ceil(-SV_L*k*dev),oy=Math.ceil(-SV_T*k*dev),
      pw=Math.max(1,ox+Math.ceil((SV_L+SV_W)*k*dev)),ph=Math.max(1,oy+Math.ceil((SV_T+SV_H)*k*dev)),
      fl=(s.np==="gate"&&s.npT<.4)?1:0;
    let sp=s._sv;
    if(!sp||sp.pw!==pw||sp.ph!==ph||sp.dev!==dev||sp.fn!==fn||sp.fl!==fl){
      const cv=document.createElement("canvas");cv.width=pw;cv.height=ph;const c=cv.getContext("2d");_svGate(c);
      let h=0;for(const ch of String(s.label||""))h=(h*31+ch.charCodeAt(0))>>>0;
      sp=s._sv={cv,c,pw,ph,dev,fn,fl,t0:-1e9,stag:(h%4)/(4*SV_HZ)};
    }
    if(gameTime-sp.t0>=1/SV_HZ||gameTime<sp.t0){
      const main=ctx;ctx=sp.c;
      try{sp.c.setTransform(1,0,0,1,0,0);sp.c.clearRect(0,0,pw,ph);
        sp.c.setTransform(dev,0,0,dev,ox,oy);fn(s,t)}
      finally{ctx=main}
      sp.t0=gameTime-sp.stag;
    }
    ctx.save();ctx.setTransform(1,0,0,1,Math.round(m.e)-ox,Math.round(m.f)-oy);ctx.drawImage(sp.cv,0,0);ctx.restore();
  }
