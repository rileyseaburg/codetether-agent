// Validate the literal macro grammar used by application-local session bindings.
// Supports one matcher arm, ident/ty/path/vis arguments and optional groups.
// Expands declaration helpers as tokens; never executes Rust or checks Rust types.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {tokens} from './rust_tokens.mjs';
// Run from the repository root. This is a bounded macro audit, not a Rust compiler.
const out='artifacts/consolidation-declaration-audit/';
fs.mkdirSync(out,{recursive:true});
const read=p=>fs.readFileSync(p,'utf8');
const walk=p=>fs.readdirSync(p,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(path.join(p,e.name)):[path.join(p,e.name)]);
const lex=s=>tokens(s).map(t=>t.value);
const pairs={'(':')','[':']','{':'}'};
function close(ts,start){const stack=[pairs[ts[start]]];assert(stack[0]);for(let i=start+1;i<ts.length;i++){if(pairs[ts[i]])stack.push(pairs[ts[i]]);else if([')',']','}'].includes(ts[i])){assert.equal(ts[i],stack.pop());if(!stack.length)return i;}}throw Error('unclosed delimiter');}
const defs=new Map();
for(const dir of fs.readdirSync('crates')){const base='crates/'+dir;if(!fs.existsSync(base+'/Cargo.toml'))continue;const owner=read(base+'/Cargo.toml').match(/^name\s*=\s*"([^"]+)"/m)?.[1]?.replaceAll('-','_');if(!owner)continue;for(const file of walk(base+'/src').filter(p=>p.endsWith('.rs'))){const ts=lex(read(file));for(let i=0;i<ts.length-4;i++)if(ts[i]==='macro_rules'&&ts[i+1]==='!'){const name=ts[i+2],end=close(ts,i+3);defs.set(owner+'::'+name,{owner,name,file,exported:ts.slice(i-4,i).join('')==='#[macro_export]',tokens:ts.slice(i+4,end)});i=end;}}}
function pattern(ts){const result=[];for(let i=0;i<ts.length;i++){if(ts[i]!=='$'){result.push({literal:ts[i]});continue;}if(ts[i+1]==='('){const end=close(ts,i+1);assert.equal(ts[end+1],'?','only optional matcher repetitions supported');result.push({optional:pattern(ts.slice(i+2,end))});i=end+1;}else {assert.equal(ts[i+2],':');assert(['ident','ty','path','vis'].includes(ts[i+3]),ts[i+3]);result.push({name:ts[i+1],kind:ts[i+3]});i+=3;}}return result;}
function valid(kind,ts){if(kind==='ident')return ts.length===1&&/^[A-Za-z_]\w*$/.test(ts[0]);if(kind==='vis')return !ts.length||ts.join('')==='pub'||/^pub\((?:crate|super|self)\)$/.test(ts.join(''));if(!ts.length)return false;let stack=[];for(const t of ts){if(['(', '[','{','<'].includes(t))stack.push(t);else if([')',']','}','>'].includes(t)){const p={')':'(',']':'[','}':'{','>':'<'}[t];if(stack.pop()!==p)return false;}else if(!stack.length&&[',',';','=','!'].includes(t))return false;}return !stack.length&&/^[A-Za-z_]/.test(ts[0]);}
function match(nodes,args,ni=0,ai=0,env={}){if(ni===nodes.length)return ai===args.length?env:null;const node=nodes[ni];if(node.literal!==undefined)return args[ai]===node.literal?match(nodes,args,ni+1,ai+1,env):null;if(node.optional){const withGroup=match([...node.optional,...nodes.slice(ni+1)],args,0,ai,env);return withGroup??match(nodes,args,ni+1,ai,env);}for(let end=ai+(node.kind==='vis'?0:1);end<=args.length;end++){const val=args.slice(ai,end);if(!valid(node.kind,val))continue;const result=match(nodes,args,ni+1,end,{...env,[node.name]:val});if(result)return result;}return null;}
function expand(key,args,chain=[]){assert(chain.length<30,'recursive macro');const d=defs.get(key);assert(d,'missing '+key);assert(d.exported,'unexported '+key);const ts=d.tokens;assert(pairs[ts[0]],key+' matcher');const pe=close(ts,0);assert.equal(ts[pe+1],'=');assert.equal(ts[pe+2],'>');const bs=pe+3,be=close(ts,bs);assert(ts.slice(be+1).every(t=>t===';'),key+' multiple arms not supported');const env=match(pattern(ts.slice(1,pe)),args);assert(env,key+' does not match '+args.join(' '));const replaced=[];for(let i=bs+1;i<be;i++){if(ts[i]==='$'){const name=ts[++i];if(name==='crate')replaced.push(d.owner);else {assert(env[name],key+' unbound '+name);replaced.push(...env[name]);}}else replaced.push(ts[i]);}return expandCalls(replaced,[...chain,key]);}
function invocation(ts,i){if(!/^codetether_/.test(ts[i]??''))return null;let k=i+1;if(ts[k]!==':'||ts[k+1]!==':')return null;k+=2;const name=ts[k++];if(ts[k++]!=='!'||!pairs[ts[k]])return null;const end=close(ts,k);return {key:ts[i]+'::'+name,args:ts.slice(k+1,end),end};}
function expandCalls(ts,chain){const result=[];for(let i=0;i<ts.length;i++){const call=invocation(ts,i);if(call&&defs.has(call.key)){result.push(...expand(call.key,call.args,chain));i=call.end;}else result.push(ts[i]);}return result;}
// Negative probes are in memory: they do not modify any repository source.
let negativeProbes=0;
function mustReject(action,label){let rejected=false;try{action();}catch{rejected=true;}assert(rejected,label);negativeProbes++;}
mustReject(()=>expand('codetether_session_schema::impl_session_lifecycle',['123']),'reject non-identifier session');
mustReject(()=>expand('codetether_session_schema::impl_session_lifecycle',['Session',',']),'reject extra argument');
mustReject(()=>expand('codetether_session_schema::missing_declaration',[]),'reject missing declaration');
const recallKey='codetether_recall_store::declare_session_host',recall=defs.get(recallKey);
const actual=lex(read('src/session/recall/storage_host.rs'));let recallCall;
for(let i=0;i<actual.length;i++){const c=invocation(actual,i);if(c?.key===recallKey){recallCall=c;break;}}
assert(recallCall);const previous=recall.tokens.join(' '),needle='$ ( , ) ?';assert(previous.includes(needle));
try{defs.set(recallKey,{...recall,tokens:lex(previous.replace(needle,''))});mustReject(()=>expand(recallKey,recallCall.args),'reject the former formatted-host mismatch');}finally{defs.set(recallKey,recall);}
const checks=[];for(const file of walk('src').filter(p=>p.endsWith('.rs'))){const ts=lex(read(file));for(let i=0;i<ts.length;i++){const call=invocation(ts,i);if(!call||!/^(?:impl_app_state_default|impl_session|impl_managed|impl_history_source|impl_projection_host|declare_session|define_session)/.test(call.key.split('::')[1]))continue;const expanded=expand(call.key,call.args);for(let j=0;j<expanded.length;j++){assert.notEqual(expanded[j],'$','unexpanded variable in '+call.key);if(pairs[expanded[j]])j=close(expanded,j);}checks.push({file,macro:call.key,expanded});i=call.end;}}
const inherent=[];for(const c of checks){const ts=c.expanded;for(let i=0;i<ts.length;i++)if(ts[i]==='impl'){let j=i+1;while(ts[j]!==undefined&&ts[j]!=='{')j++;assert(ts[j],c.macro);if(ts.slice(i+1,j).includes('for'))continue;const end=close(ts,j);let depth=0;for(let k=j+1;k<end;k++){if(ts[k]==='{')depth++;if(ts[k]==='}')depth--;if(!depth&&ts[k]==='fn')inherent.push({name:ts[k+1],file:c.file,macro:c.macro});}i=end;}}
const grouped=new Map();for(const m of inherent){const key=m.name;assert(!grouped.has(key),'duplicate inherent Session method '+key);grouped.set(key,m);}
fs.writeFileSync(out+'expanded.json',JSON.stringify(checks));fs.writeFileSync(out+'methods.json',JSON.stringify(inherent,null,2));
console.log(JSON.stringify({negativeProbes,invocations:checks.length,macroDefinitions:defs.size,expandedTokens:checks.reduce((n,c)=>n+c.expanded.length,0),inherentSessionMethods:inherent.length,unboundVariables:0,duplicateInherentMethods:0,coverage:'limited declaration-macro grammar; not Rust type checking'}));
