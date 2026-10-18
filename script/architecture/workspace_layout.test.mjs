import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {spawnSync} from 'node:child_process';

const repository = fileURLToPath(new URL('../..', import.meta.url));
const script = path.join(repository, 'script/architecture/check_workspace_layout.mjs');
const evidence = path.join(repository, 'artifacts/workspace-layout-tests');
fs.mkdirSync(evidence, {recursive: true});
function write(root, name, content) {
  const file = path.join(root, name);
  fs.mkdirSync(path.dirname(file), {recursive: true});
  fs.writeFileSync(file, content);
}
function fixture() {
  const root = fs.mkdtempSync(path.join(evidence, 'case-'));
  write(root, 'Cargo.toml', '[workspace]\ndefault-members = ["crates/codetether-agent"]\nmembers = ["crates/codetether-agent", "crates/helper"]\n');
  write(root, 'crates/codetether-agent/Cargo.toml', '[package]\nname = "codetether-agent"\nversion = "1.0.0"\n[dependencies]\nalias = { package = "helper", path = "../helper" }\n');
  write(root, 'crates/helper/Cargo.toml', '[package]\nname = "helper"\nversion = "1.0.0"\n');
  for (const file of ['src/lib.rs', 'src/main.rs', 'examples/browser_validate.rs',
    'examples/oracle_demo.rs', 'examples/thinker_example.rs']) {
    write(root, 'crates/codetether-agent/' + file, '');
  }
  return root;
}
function check(root) {
  const result = spawnSync(process.execPath, [script], {cwd: root, encoding: 'utf8'});
  write(root, 'check.log', result.stdout + result.stderr);
  return result;
}
test('audits aliased workspace dependencies independently of default members', () => {
  const result = check(fixture());
  assert.equal(result.status, 0, result.stderr);
  const report = JSON.parse(result.stdout);
  assert.equal(report.packages, 2);
  assert.equal(report.productionDependencyEdges, 1);
});
test('rejects a dependency cycle', () => {
  const root = fixture();
  fs.appendFileSync(path.join(root, 'crates/helper/Cargo.toml'),
    '[dependencies]\ncodetether-agent = { path = "../codetether-agent" }\n');
  const result = check(root);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Dependency cycle/);
});
test('rejects an incorrect dependency package identity', () => {
  const root = fixture();
  const file = path.join(root, 'crates/codetether-agent/Cargo.toml');
  fs.writeFileSync(file, fs.readFileSync(file, 'utf8').replace('package = "helper"', 'package = "wrong"'));
  const result = check(root);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Dependency identity mismatch/);
});
test('rejects a missing literal test-source include', () => {
  const root = fixture();
  write(root, 'crates/codetether-agent/tests/missing.rs', '#[path = "absent.rs"]\nmod absent;\n');
  const result = check(root);
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Missing test source/);
});
