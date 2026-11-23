const fs=require("fs");const {JSDOM,VirtualConsole}=require("jsdom");
const SRC=process.argv[2]||"/home/claude/htha/index.html";
const STAGES=(process.argv[3]||"1,100,500,1500").split(",").map(Number);
const FRAMES=+process.argv[4]||2400;
let html=fs.readFileSync(SRC,"utf8");
const HOOK=`\n  window.__dbg={update,draw,resetRun,applyClass,startStage,setGameUI,
    setStage:v=>{stage=v},setRunning:v=>{running=v;paused=false},getStage:()=>stage,
    counts:()=>({enemies:enemies.length,shots:shots.length,particles:(typeof particles!=="undefined"?particles.length:0),
      texts:(typeof floatTexts!=="undefined"?floatTexts.length:0),hazards:hazards.length,hearts:hearts.length,summons:summons.length}),
    setQ:v=>{Q=v;resize()},useSkill:i=>useSkill(i),clearCD:()=>{for(let i=0;i<skillCD.length;i++)skillCD[i]=0},classKeys:()=>Object.keys(classes),getQ:()=>Q,god:()=>{player.invuln=1e9;player.hp=player.maxHp;player.mana=player.maxMana}};\n  requestAnimationFrame(loop);\n`;
const marker="\n  requestAnimationFrame(loop);\n";
if(html.split(marker).length!==2)throw new Error("marker không duy nhất: "+(html.split(marker).length-1));
html=html.replace(marker,HOOK);

const counts={},sets={shadowBlur:0,filter:0,gco:0};
const stats={createGradient:0};
function makeCtx(canvas){
  const base={canvas,measureText:()=>({width:10}),getImageData:()=>({data:new Uint8ClampedArray(4)}),
    createPattern:()=>null,isPointInPath:()=>false};
  const grad={addColorStop(){counts.addColorStop=(counts.addColorStop||0)+1}};
  return new Proxy(base,{
    get(t,p){ if(p in t)return t[p];
      if(typeof p==="symbol")return undefined;
      if(p==="createRadialGradient"||p==="createLinearGradient"||p==="createConicGradient")return(...a)=>{counts[p]=(counts[p]||0)+1;stats.createGradient++;if(ATTR){const c=caller();GRAD_BY[c]=(GRAD_BY[c]||0)+1}return grad};
      if(/^(fillStyle|strokeStyle|lineWidth|globalAlpha|font|textAlign|textBaseline|lineCap|lineJoin|shadowColor|shadowBlur|filter|globalCompositeOperation|imageSmoothingEnabled)$/.test(p))return t["_"+p];
      return (...a)=>{counts[p]=(counts[p]||0)+1;if(ATTR){const c=caller();BYFN[c]=(BYFN[c]||0)+1}};},
    set(t,p,v){ t["_"+p]=v;
      if(p==="shadowBlur"&&v>0){sets.shadowBlur++;if(ATTR){const c=caller();SHADOW_BY[c]=(SHADOW_BY[c]||0)+1}} if(p==="filter"&&v&&v!=="none")sets.filter++;
      if(p==="globalCompositeOperation"&&v!=="source-over")sets.gco++; return true}
  });
}
const BYFN={},SHADOW_BY={},GRAD_BY={};let ATTR=false;
let CERR=0;function caller(){try{return caller0()}catch(e){CERR++;return"?err"}}
function caller0(){const st=new Error().stack.split("\n");for(let i=3;i<st.length;i++){const m=st[i].match(/at (?:Object\.)?([\w$]+) \(/);if(m&&!/^(get|set|apply|Proxy|eval)$/.test(m[1]))return m[1]}return"?"}
const METHODS="save restore beginPath closePath moveTo lineTo quadraticCurveTo bezierCurveTo arc arcTo ellipse rect fill stroke clip fillRect strokeRect clearRect fillText strokeText drawImage translate rotate scale transform setTransform resetTransform setLineDash putImageData".split(" ");
let CALLS=0;const noop=function(){CALLS++};const gradObj={addColorStop(){}};
function makeFast(canvas){const c={canvas,measureText:()=>({width:10}),getImageData:()=>({data:new Uint8ClampedArray(4)}),createPattern:()=>({}),
  createRadialGradient:()=>gradObj,createLinearGradient:()=>gradObj,createConicGradient:()=>gradObj,isPointInPath:()=>false,getTransform:()=>({a:1,b:0,c:0,d:1,e:0,f:0}),
  fillStyle:"",strokeStyle:"",lineWidth:1,globalAlpha:1,font:"",textAlign:"",textBaseline:"",lineCap:"",lineJoin:"",shadowColor:"",shadowBlur:0,filter:"none",globalCompositeOperation:"source-over",imageSmoothingEnabled:true};
  for(const m of METHODS)c[m]=noop;return c}
const errors=[];
const vc=new VirtualConsole();vc.on("jsdomError",e=>errors.push(String(e.message||e).slice(0,200)));
vc.on("error",e=>errors.push("console.error: "+String(e).slice(0,200)));
const dom=new JSDOM(html,{runScripts:"dangerously",pretendToBeVisual:true,url:"https://localhost/",virtualConsole:vc,
  beforeParse(w){
    if(process.env.ANDROID!=="0")Object.defineProperty(w.navigator,"userAgent",{get:()=>"Mozilla/5.0 (Linux; Android 11; SM-A217F) AppleWebKit/537.36 Chrome/120 Mobile Safari/537.36"});
    Object.defineProperty(w.navigator,"deviceMemory",{get:()=>6});Object.defineProperty(w.navigator,"hardwareConcurrency",{get:()=>8});
    globalThis.__vt=0;w.performance.now=()=>globalThis.__vt;
    w.HTMLCanvasElement.prototype.getContext=function(){return this.__c||(this.__c=makeFast(this))};
    w.HTMLCanvasElement.prototype.toDataURL=()=>"data:,";
    w.matchMedia=w.matchMedia||(()=>({matches:false,addEventListener(){},removeEventListener(){},addListener(){}}));
    w.ResizeObserver=class{observe(){}disconnect(){}};
    const q=[];w.__q=q;w.requestAnimationFrame=cb=>{q.push(cb);return q.length};w.cancelAnimationFrame=()=>{};
    w.AudioContext=w.webkitAudioContext=class{constructor(){this.destination={};this.currentTime=0;this.state="running"}
      createGain(){return{gain:{value:1,setValueAtTime(){},linearRampToValueAtTime(){},exponentialRampToValueAtTime(){}},connect(){},disconnect(){}}}
      createOscillator(){return{frequency:{value:0,setValueAtTime(){},exponentialRampToValueAtTime(){}},type:"",connect(){},start(){},stop(){},disconnect(){}}}
      createBuffer(){return{getChannelData:()=>new Float32Array(1)}}createBufferSource(){return{connect(){},start(){},stop(){},buffer:null,playbackRate:{value:1}}}
      createBiquadFilter(){return{frequency:{value:0,setValueAtTime(){}},Q:{value:0},type:"",connect(){}}}resume(){return Promise.resolve()}decodeAudioData(){return Promise.resolve({})}};
    w.fetch=()=>Promise.reject(new Error("offline"));
    w.HTMLMediaElement.prototype.play=()=>Promise.resolve();w.HTMLMediaElement.prototype.pause=()=>{};
  }});
const w=dom.window;performance.now_real=()=>Number(process.hrtime.bigint())/1e6;
setTimeout(async()=>{
  const d=w.__dbg;
  if(!d){console.log("KHÔNG gắn được hook. Lỗi:\n"+errors.slice(0,8).join("\n"));process.exit(2)}
  const pct=(a,p)=>{const s=[...a].sort((x,y)=>x-y);return s[Math.min(s.length-1,Math.floor(s.length*p))]};
  const out=[];
  for(const st of STAGES){ATTR=false;
    for(const k of Object.keys(counts))delete counts[k];Object.assign(sets,{shadowBlur:0,filter:0,gco:0});stats.createGradient=0;
    w.localStorage.clear();
    d.setQ(+(process.env.Q||0));d.resetRun(process.env.CLS||"archer");d.setStage(st);d.applyClass();d.startStage();d.setGameUI(true);d.setRunning(true);
    const U=[],D=[];let maxE=0,maxS=0,maxP=0,maxT=0,drawCalls=0,frames=0;
    for(let f=0;f<FRAMES;f++){
      d.god();const dt=1/60;
      if(process.env.SKILLS==='1'&&f%12===0){d.clearCD();for(let i=0;i<3;i++){try{d.useSkill(i)}catch(e){}}}
      globalThis.__vt+=16.7;const t0=performance.now_real();d.update(dt);const t1=performance.now_real();
      const before=CALLS;
      d.draw();const t2=performance.now_real();
      const after=CALLS;
      if(f===300&&process.env.ATTR!=="0")ATTR=true;
      if(f>=300){U.push(t1-t0);D.push(t2-t1);drawCalls+=after-before;frames++;}
      const c=d.counts();maxE=Math.max(maxE,c.enemies);maxS=Math.max(maxS,c.shots);maxP=Math.max(maxP,c.particles);maxT=Math.max(maxT,c.texts);
    }
    const mean=a=>a.reduce((x,y)=>x+y,0)/a.length;
    out.push({stage:st,reached:d.getStage(),frames,
      update_ms:{mean:+mean(U).toFixed(3),p95:+pct(U,.95).toFixed(3),max:+Math.max(...U).toFixed(2)},
      draw_js_ms:{mean:+mean(D).toFixed(3),p95:+pct(D,.95).toFixed(3),max:+Math.max(...D).toFixed(2)},
      canvas_calls_per_frame:Math.round(drawCalls/frames),gradients_per_frame:+(stats.createGradient/(FRAMES)).toFixed(1),
      shadowBlur_sets_per_frame:+(sets.shadowBlur/FRAMES).toFixed(1),filter_sets_per_frame:+(sets.filter/FRAMES).toFixed(2),
      peak:{enemies:maxE,shots:maxS,particles:maxP,floatTexts:maxT},
      top_calls:Object.entries(counts).sort((a,b)=>b[1]-a[1]).slice(0,8).map(([k,v])=>k+":"+Math.round(v/FRAMES))});
  }
  const top=(o,n=12)=>Object.entries(o).sort((a,b)=>b[1]-a[1]).slice(0,n).map(([k,v])=>k+":"+Math.round(v/(FRAMES-300)*10)/10);
  console.log("CALLS/frame theo hàm:",top(BYFN).join("  "));console.log("SHADOW/frame theo hàm:",top(SHADOW_BY).join("  "));console.log("GRADIENT/frame theo hàm:",top(GRAD_BY).join("  "));
  console.log("caller errors:",CERR,"sumBYFN/frame:",Math.round(Object.values(BYFN).reduce((a,b)=>a+b,0)/(FRAMES-300)));console.log(JSON.stringify(out.map(r=>({stage:r.stage,calls:r.canvas_calls_per_frame,update:r.update_ms,draw:r.draw_js_ms,peak:r.peak}))));
  if(errors.length)console.log("jsdom errors (đầu):",errors.slice(0,5));
  process.exit(0);
},1500);
