// Audit direct production dependency scope for native crate source modules.
// This follows literal modules/includes and excludes explicit test gates.
// Feature/target dependencies are a conservative union. Macros are not expanded;
// this is not Rust name resolution or type checking.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {tokens} from './rust_tokens.mjs';

const pairs = {'(': ')', '[': ']', '{': '}'};
function close(ts, start) {
  const stack = [pairs[ts[start].value]];
  assert(stack[0]);
  for (let i = start + 1; i < ts.length; i++) {
    const value = ts[i].value;
    if (pairs[value]) stack.push(pairs[value]);
    else if ([')', ']', '}'].includes(value)) {
      assert.equal(value, stack.pop());
      if (!stack.length) return i;
    }
  }
  throw new Error('Unclosed source delimiter');
}
function items(source) {
  const ts = tokens(source), out = [];
  let start = 0;
  for (let i = 0; i < ts.length; i++) {
    if (pairs[ts[i].value]) i = close(ts, i);
    if (ts[i].value !== ';' && ts[i].value !== '}') continue;
    if (ts[i + 1]?.value === ';') i++;
    const part = ts.slice(start, i + 1);
    let keyword = -1;
    for (let k = 0; k < part.length; k++) {
      if (['use','mod','impl','trait','struct','enum','fn','macro_rules','const','type','extern'].includes(part[k].value)) { keyword = k; break; }
      if (pairs[part[k].value]) { k = close(part, k); continue; }
    }
    let attributeEnd = part[0].start;
    for (let k = 0; part[k]?.value === '#' && part[k + 1]?.value === '[';) {
      k = close(part, k + 1);
      attributeEnd = part[k++].end;
    }
    const prefix = source.slice(part[0].start, keyword < 0 ? attributeEnd : part[keyword].start);
    out.push({ts: part, kind: keyword < 0 ? '' : part[keyword].value, keyword, prefix});
    start = i + 1;
  }
  assert.equal(start, ts.length, 'Unsupported trailing source tokens');
  return out;
}
function testOnly(prefix) {
  return /#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]/.test(prefix)
    || /#\s*\[\s*cfg\s*\(\s*all\s*\(\s*test\s*[,)]/.test(prefix)
    || /#\s*\[\s*(?:tokio\s*::\s*)?test\s*\]/.test(prefix);
}
function dependencies(manifest) {
  const production = new Set(), development = new Set();
  let target;
  for (const raw of manifest.split('\n')) {
    const section = raw.match(/^\[([^\]]+)\]/)?.[1];
    if (section !== undefined) {
      target = /(?:^|\.)dev-dependencies(?:\.|$)/.test(section) ? development
        : /(?:^|\.)dependencies(?:\.|$)/.test(section) && !section.includes('build-dependencies') ? production : undefined;
      const tableDependency = section.match(/(?:^|\.)(?:dev-)?dependencies\.([\w-]+)$/)?.[1];
      if (target && tableDependency) target.add(tableDependency.replaceAll('-', '_'));
      continue;
    }
    const name = raw.match(/^([\w-]+)\s*=/)?.[1];
    if (target && name && !['path','version','features','optional','default-features','package'].includes(name)) target.add(name.replaceAll('-', '_'));
  }
  return {production, development};
}
assert(testOnly('#[cfg(test)]'));
assert(testOnly('#[cfg(all(test, unix))]'));
assert(!testOnly('#[cfg(not(test))]'));
assert(!testOnly('#[cfg(any(test, feature = "test-support"))]'));
assert(testOnly(items('#[cfg(test)] include!("tests.rs");')[0].prefix));
assert(!testOnly(items('include!("production.rs");')[0].prefix));
const probe = dependencies('[dependencies]\nprod = "1"\n[dev-dependencies]\nfixture = "1"\n[target.\'cfg(windows)\'.dependencies]\nplatform = "1"\n[build-dependencies]\nbuilder = "1"');
assert(probe.production.has('prod') && probe.production.has('platform') && !probe.production.has('fixture') && !probe.production.has('builder'));
assert(probe.development.has('fixture'));

const workspace = fs.readdirSync('crates').filter(name => fs.existsSync(`crates/${name}/Cargo.toml`));
const known = new Set(workspace.map(name => name.replaceAll('-', '_')));
const selected = workspace.filter(name => name.startsWith('codetether-native-'));
const findings = [], unsupported = [], reports = [];
for (const name of selected) {
  const root = `crates/${name}/src`, own = name.replaceAll('-', '_');
  const {production, development} = dependencies(fs.readFileSync(`crates/${name}/Cargo.toml`, 'utf8'));
  const seen = new Set(), uses = new Map();
  function inspect(ts, file, kind) {
    for (let i = 0; i < ts.length; i++) {
      const value = ts[i].value;
      const rootUse = ts[i + 1]?.value === ':' && ts[i + 2]?.value === ':' || ts[i - 1]?.value === 'use';
      if (value !== own && rootUse && (known.has(value) || development.has(value)) && !production.has(value)) {
        if (!uses.has(value)) uses.set(value, []);
        uses.get(value).push({file, kind});
      }
    }
  }
  function visit(file, inheritedBase) {
    file = path.normalize(file);
    if (seen.has(file)) return;
    seen.add(file);
    if (!fs.existsSync(file)) { unsupported.push({name, file, reason: 'missing module'}); return; }
    const source = fs.readFileSync(file, 'utf8');
    const directory = path.dirname(file);
    const base = inheritedBase ?? (['lib.rs','mod.rs'].includes(path.basename(file)) ? directory : file.slice(0, -3));
    function scan(source, moduleBase, attributeBase) {
      for (const item of items(source)) {
        if (testOnly(item.prefix)) continue;
        const {ts, kind, keyword} = item;
        if (['mod','impl','trait'].includes(kind)) {
          const open = ts.findIndex((t, i) => i > keyword && t.value === '{');
          if (open >= 0) {
            inspect(ts.slice(0, open), file, kind);
            const end = close(ts, open);
            const nested = source.slice(ts[open].end, ts[end].start);
            const nestedBase = kind === 'mod' ? path.join(moduleBase, ts[keyword + 1].value) : moduleBase;
            scan(nested, nestedBase, kind === 'mod' ? path.join(attributeBase, ts[keyword + 1].value) : attributeBase);
          } else if (kind === 'mod') {
            const explicit = item.prefix.match(/#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]/)?.[1];
            const child = ts[keyword + 1].value;
            const candidates = explicit ? [path.join(attributeBase, explicit)] : [path.join(moduleBase, child+'.rs'), path.join(moduleBase, child, 'mod.rs')];
            visit(candidates.find(fs.existsSync) ?? candidates[0]);
          } else inspect(ts, file, kind);
        } else {
          inspect(ts, file, kind);
          for (let i=0; i<ts.length-3; i++) if (ts[i].value==='include' && ts[i+1].value==='!' && ts[i+2].value==='(') {
            if (/^".*"$/.test(ts[i+3].value)) visit(path.join(directory, JSON.parse(ts[i+3].value)), moduleBase);
            else unsupported.push({name, file, reason:'nonliteral include'});
          }
        }
      }
    }
    try { scan(source, base, directory); } catch(error) { unsupported.push({name, file, reason:error.message}); }
  }
  visit(root+'/lib.rs');
  reports.push({name, productionFiles: seen.size});
  for (const [dependency, locations] of uses) findings.push({name, dependency, scope: development.has(dependency) ? 'dev-only' : 'undeclared', locations: [...new Map(locations.map(x=>[x.file+':'+x.kind,x])).values()]});
}
const result = {coverage:'native crate literal module graph; explicit test gates excluded; no macro expansion or Rust type checking',nativeCrates:selected.length,productionFiles:reports.reduce((n,r)=>n+r.productionFiles,0),findings,unsupported};
const out='artifacts/consolidation-production-dependencies';
fs.mkdirSync(out,{recursive:true});
fs.writeFileSync(out+'/report.json',JSON.stringify({...result,reports},null,2));
console.log(JSON.stringify(result,null,2));
if (findings.length || unsupported.length) process.exitCode=1;
