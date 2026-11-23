const fs=require("fs");const {createCanvas}=require("@napi-rs/canvas");
const html=fs.readFileSync("/home/claude/htha/index.html","utf8");
const L=html.split("\n");
const a=L.findIndex(l=>l.includes("SERVANT MODELS"))-1, j=L.findIndex((l,i)=>i>a&&/^  function drawFx\(\)/.test(l));
const models=L.slice(a,j).join("\n");
const c0=L.findIndex(l=>l.includes("BƯỚC 3 — HIỆU NĂNG")), c1=L.findIndex((l,i)=>i>c0&&l.includes("SERVANT MODELS")) -1;
const cache=L.slice(c0,c1).join("\n");
const mk=new Function("TAU","document","localStorage","canvas",
 "let ctx=null,gameTime=0,W=400,Q=2;\n"+models+"\n"+cache+"\nreturn {setCtx:c=>{ctx=c},setT:v=>{gameTime=v},setW:v=>{W=v},setQ:v=>{Q=v},drawArtoria,drawGilgamesh,drawEnkidu,drawServantModel};");
const SZ=360;
function run(name,r,dev,fx,fy,tDraw,tCache,opts={}){
  const main=createCanvas(Math.round(SZ*dev),Math.round(SZ*dev));
  const doc={createElement:()=>createCanvas(1,1)};
  const M=mk(Math.PI*2,doc,{getItem:()=>null},main);
  const fn=M["draw"+name];
  const s={label:name,r,np:name==="Gilgamesh"?"gate":name==="Artoria"?"excal":"enuma",npT:opts.npT??9,x:0,y:0};
  const mkImg=(cb)=>{const c=createCanvas(main.width,main.height),x=c.getContext("2d");x.fillStyle="#303040";x.fillRect(0,0,c.width,c.height);M.setCtx(x);x.setTransform(dev,0,0,dev,0,0);x.save();x.translate(SZ/2+fx,SZ/2+fy);cb();x.restore();return x.getImageData(0,0,c.width,c.height).data};
  M.setW(SZ);
  const direct=mkImg(()=>fn(s,tDraw));
  // đường cache: render lần đầu tại tCache, sau đó hiển thị ở tDraw (nếu tDraw-tCache < 1/15 thì dùng lại sprite cũ)
  M.setQ(2);
  const s2={...s};
  mkImg(()=>{M.setT(tCache);M.drawServantModel(s2,tCache,fn)});             // làm mới sprite tại tCache
  const cached=mkImg(()=>{M.setT(tDraw);M.drawServantModel(s2,tDraw,fn)});   // hiển thị tại tDraw
  let sum=0,big=0,max=0,n=0,cov=0;
  for(let i=0;i<direct.length;i+=4){
    const d=Math.max(Math.abs(direct[i]-cached[i]),Math.abs(direct[i+1]-cached[i+1]),Math.abs(direct[i+2]-cached[i+2]));
    const isBg=(a)=>a[i]===0x30&&a[i+1]===0x30&&a[i+2]===0x40;
    if(!isBg(direct)||!isBg(cached)){cov++;sum+=d;if(d>24)big++;}max=Math.max(max,d);n++}
  return {covPx:cov,meanDiff:+(sum/Math.max(1,cov)).toFixed(2),pctBig:+(100*big/Math.max(1,cov)).toFixed(2),max,refreshed:s2._sv&&s2._sv.t0};
}
const out=[];
console.log("A) Cùng thời điểm, đặt ở TỌA ĐỘ NGUYÊN (chỉ khác đường vẽ) — kỳ vọng ≈ 0");
for(const name of ["Artoria","Gilgamesh","Enkidu"])for(const dev of [1,1.5])for(const r of [14,18,26]){
  const m=run(name,r,dev,0,0,3.21,3.21);console.log(" ",name.padEnd(9),"dev",dev,"r",r,JSON.stringify(m))}
console.log("B) Cùng thời điểm, tọa độ LẺ (0.37, 0.61) — sai khác do nội suy ảnh");
for(const name of ["Artoria","Gilgamesh","Enkidu"]){const m=run(name,18,1,.37,.61,3.21,3.21);console.log(" ",name.padEnd(9),JSON.stringify(m))}
console.log("C) Độ trễ hoạt họa: sprite cũ 66ms so với vẽ trực tiếp — và so sánh với mức đổi giữa 2 khung liền kề (16.7ms)");
for(const name of ["Artoria","Gilgamesh","Enkidu"]){
  const lag=run(name,18,1,0,0,3.21+.066,3.21),base=run(name,18,1,0,0,3.21+.0167,3.21);
  console.log(" ",name.padEnd(9),"trễ 66ms:",JSON.stringify({mean:lag.meanDiff,big:lag.pctBig}),"| khung kề 16.7ms:",JSON.stringify({mean:base.meanDiff,big:base.pctBig}))}
console.log("D) Tia Gilgamesh (np=gate, npT<.4) vs không:");
{const a=run("Gilgamesh",18,1,0,0,3.21,3.21,{npT:.2}),b=run("Gilgamesh",18,1,0,0,3.21,3.21,{npT:9});console.log("  flash:",JSON.stringify(a),"| không flash:",JSON.stringify(b))}
console.log("E) KIỂM TRA GIẢ THUYẾT: r=20 (k=1), dev=1 và 2, tọa độ nguyên — gốc sprite trùng lưới điểm ảnh");
for(const name of ["Artoria","Gilgamesh","Enkidu"])for(const dev of [1,2]){const m=run(name,20,dev,0,0,3.21,3.21);console.log(" ",name.padEnd(9),"dev",dev,JSON.stringify(m))}
