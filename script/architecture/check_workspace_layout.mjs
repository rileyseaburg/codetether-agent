// Structural workspace audit. This does not compile Rust or resolve Rust types.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {applicationLayout} from './application_layout.mjs';

const application = applicationLayout();
assert.equal(application.directory, 'crates/codetether-agent');
assert(!fs.existsSync('src'), 'Root source tree must remain consolidated');
assert(!fs.existsSync('tests'), 'Application tests must remain with their package');
const root = fs.readFileSync('Cargo.toml', 'utf8');
assert(!/^\[package\]/m.test(root), 'Repository root must be a virtual workspace');
const members = [...root.match(/^members\s*=\s*\[([\s\S]*?)\]/m)[1].matchAll(/"([^"]+)"/g)]
  .map(match => match[1]);
assert(root.includes('default-members = ["crates/codetether-agent"]'));

function walk(directory) {
  if (!fs.existsSync(directory)) return [];
  return fs.readdirSync(directory, {withFileTypes: true}).flatMap(entry => {
    const file = path.join(directory, entry.name);
    return entry.isDirectory() ? walk(file) : [file];
  });
}
const packages = new Map();
const names = new Set();
for (const member of members) {
  const manifest = path.resolve(member, 'Cargo.toml');
  assert(fs.existsSync(manifest), `Missing workspace member: ${member}`);
  const text = fs.readFileSync(manifest, 'utf8');
  const section = text.match(/^\[package\]\s*\n([\s\S]*?)(?=^\[|$(?![\s\S]))/m)?.[1];
  const name = section?.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
  assert(name, `Missing package identity: ${member}`);
  assert(!names.has(name), `Duplicate package identity: ${name}`);
  names.add(name);
  packages.set(manifest, {member, name, text, dependencies: []});
}
for (const entry of fs.readdirSync('crates', {withFileTypes: true})) {
  const manifest = path.resolve('crates', entry.name, 'Cargo.toml');
  if (entry.isDirectory() && fs.existsSync(manifest)) {
    assert(packages.has(manifest), `Unregistered crate package: ${entry.name}`);
  }
}

let developmentDependencyEdges = 0;
for (const [manifest, pkg] of packages) {
  let section = '';
  for (const line of pkg.text.split('\n')) {
    const header = line.match(/^\[([^\]]+)\]/)?.[1];
    if (header) section = header;
    if (!/(?:^|\.)(?:dependencies|dev-dependencies|build-dependencies)$/.test(section)) continue;
    const dependency = line.match(/^([\w-]+)\s*=\s*\{(.*)\}\s*$/);
    const localPath = dependency?.[2].match(/\bpath\s*=\s*"([^"]+)"/)?.[1];
    if (!localPath) continue;
    const targetPath = path.resolve(path.dirname(manifest), localPath, 'Cargo.toml');
    assert(fs.existsSync(targetPath), `Missing dependency: ${pkg.name} -> ${localPath}`);
    const target = packages.get(targetPath);
    assert(target, `Local dependency outside workspace: ${pkg.name} -> ${localPath}`);
    const requested = dependency[2].match(/\bpackage\s*=\s*"([^"]+)"/)?.[1] ?? dependency[1];
    assert.equal(target.name, requested, `Dependency identity mismatch in ${pkg.name}`);
    if (/(?:^|\.)dev-dependencies$/.test(section)) developmentDependencyEdges++;
    else pkg.dependencies.push(targetPath);
  }
}
const visited = new Set();
function visit(manifest, stack = []) {
  assert(!stack.includes(manifest), `Dependency cycle: ${[...stack, manifest].join(' -> ')}`);
  if (visited.has(manifest)) return;
  for (const dependency of packages.get(manifest).dependencies) visit(dependency, [...stack, manifest]);
  visited.add(manifest);
}
for (const manifest of packages.keys()) visit(manifest);

const files = walk(application.source);
const testSources = walk(path.join(application.directory, 'tests')).filter(file => file.endsWith('.rs'));
let literalTestLinks = 0;
for (const file of testSources) {
  const source = fs.readFileSync(file, 'utf8');
  for (const match of source.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)) {
    assert(fs.existsSync(path.resolve(path.dirname(file), match[1])), `Missing test source: ${file}: ${match[1]}`);
    literalTestLinks++;
  }
}
for (const file of ['src/lib.rs', 'src/main.rs', 'examples/browser_validate.rs',
  'examples/oracle_demo.rs', 'examples/thinker_example.rs']) {
  assert(fs.existsSync(path.join(application.directory, file)), `Missing application target: ${file}`);
}
const report = {
  scope: 'Literal workspace membership, production/build dependency paths and cycles, application layout, test source paths; no compilation',
  application,
  packages: packages.size,
  productionDependencyEdges: [...packages.values()].reduce((total, pkg) => total + pkg.dependencies.length, 0),
  developmentDependencyEdges,
  sourceFiles: files.filter(file => file.endsWith('.rs')).length,
  integrationTestSources: testSources.length,
  literalTestLinks,
  ownership: [...packages.values()].map(({member, name}) => ({member, name})),
};
if (process.argv[2]) fs.writeFileSync(process.argv[2], JSON.stringify(report, null, 2) + '\n');
console.log(JSON.stringify({...report, ownership: `${report.ownership.length} package owners`}));
