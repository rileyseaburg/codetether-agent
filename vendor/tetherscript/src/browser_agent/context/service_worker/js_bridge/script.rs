//! JavaScript source for the deterministic service-worker bridge.

use crate::browser_agent::keyboard_escape::quote;

pub(super) fn install(origin: &str, registrations: &str, caches: &str) -> String {
    format!(
        "window.__agentSw={{origin:{},regs:{registrations},caches:{caches},ops:[]}};{}",
        quote(origin),
        BRIDGE
    )
}

pub(super) fn drain() -> &'static str {
    "let __swOps=[];if(window.__agentSw){__swOps=window.__agentSw.ops;window.__agentSw.ops=[];}__swOps;"
}

const BRIDGE: &str = "function __swP(v){return Promise_resolve(v);}function __swHas(xs,v){for(let i=0;i<xs.length;i=i+1){if(xs[i]==v){return true;}}return false;}function __swReg(r){if(!r){return undefined;}let o={origin:r.origin,scope:r.scope,scriptURL:r.scriptURL,state:r.state};if(r.state=='active'){o.active={state:'activated',scriptURL:r.scriptURL};}else{o.installing={state:r.state,scriptURL:r.scriptURL};}return o;}function __swFind(c,u){let s=window.__agentSw;for(let i=0;i<s.caches.length;i=i+1){let r=s.caches[i];if((c==undefined||r.cacheName==c)&&(r.requestURL==u||r.requestPath==u||r.requestName==u)){return r;}}return undefined;}function __swResp(r){if(!r){return undefined;}let o={status:r.status,ok:r.status>=200&&r.status<300,url:r.requestURL,origin:r.origin,cacheName:r.cacheName};o.text=function(){return __swP(r.body);};return o;}function __swCache(n){let c={name:n};c.match=function(u){return __swP(__swResp(__swFind(n,''+u)));};c.delete=function(u){let s=window.__agentSw;let next=[];let ok=false;for(let i=0;i<s.caches.length;i=i+1){let r=s.caches[i];if(r.cacheName==n&&(r.requestURL==u||r.requestPath==u||r.requestName==u)){ok=true;}else{next.push(r);}}s.caches=next;if(ok){s.ops.push(['cacheDelete',n,''+u]);}return __swP(ok);};c.keys=function(){let s=window.__agentSw;let out=[];for(let i=0;i<s.caches.length;i=i+1){let r=s.caches[i];if(r.cacheName==n){out.push(r.requestURL);}}return __swP(out);};return c;}window.caches={open:function(n){return __swP(__swCache(''+n));},match:function(u){return __swP(__swResp(__swFind(undefined,''+u)));},delete:function(n){let s=window.__agentSw;let next=[];let ok=false;for(let i=0;i<s.caches.length;i=i+1){let r=s.caches[i];if(r.cacheName==n){ok=true;}else{next.push(r);}}s.caches=next;if(ok){s.ops.push(['cacheStorageDelete',''+n]);}return __swP(ok);},keys:function(){let s=window.__agentSw;let out=[];for(let i=0;i<s.caches.length;i=i+1){let n=s.caches[i].cacheName;if(!__swHas(out,n)){out.push(n);}}return __swP(out);}};var caches=window.caches;navigator.serviceWorker={register:function(url,opt){let s=window.__agentSw;let scope='/';if(opt&&opt.scope){scope=''+opt.scope;}let r={origin:s.origin,scope:s.origin+scope,scriptURL:s.origin+url,state:'installing'};s.regs.push(r);s.ops.push(['register',scope,''+url]);return __swP(__swReg(r));},getRegistration:function(scope){let s=window.__agentSw;let want='';if(scope){want=s.origin+scope;}for(let i=0;i<s.regs.length;i=i+1){let r=s.regs[i];if(want==''||r.scope==want){return __swP(__swReg(r));}}return __swP(undefined);},ready:__swP((function(){let s=window.__agentSw;for(let i=0;i<s.regs.length;i=i+1){if(s.regs[i].state=='active'){return __swReg(s.regs[i]);}}return undefined;})())};";
