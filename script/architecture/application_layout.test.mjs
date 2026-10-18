import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {execFileSync, spawnSync} from 'node:child_process';
import {applicationLayout} from './application_layout.mjs';

const repository = fileURLToPath(new URL('../..', import.meta.url));
const evidence = path.join(repository, 'artifacts/application-layout-tests');
fs.mkdirSync(evidence, {recursive: true});
function fixture() { return fs.mkdtempSync(path.join(evidence, 'case-')); }
function write(root, name, text) {
  const target = path.join(root, name);
  fs.mkdirSync(path.dirname(target), {recursive: true});
  fs.writeFileSync(target, text);
}
function application(root, directory) {
  write(root, `${directory}/Cargo.toml`, '[package]\nname = "codetether-agent"\nversion = "1.0.0"\n');
  write(root, `${directory}/src/lib.rs`, '');
}
test('locates the current package and relocated package', () => {
  const current = fixture();
  application(current, '.');
  assert.equal(applicationLayout(current).source, 'src');
  const moved = fixture();
  write(moved, 'Cargo.toml', '[workspace]\nmembers = ["crates/codetether-agent"]\n');
  application(moved, 'crates/codetether-agent');
  assert.equal(applicationLayout(moved).source, 'crates/codetether-agent/src');
});
test('rejects missing and ambiguous application packages', () => {
  const root = fixture();
  assert.throws(() => applicationLayout(root), /found 0/);
  application(root, '.');
  application(root, 'crates/codetether-agent');
  assert.throws(() => applicationLayout(root), /found 2/);
});
test('does not select another package merely because it has source files', () => {
  const root = fixture();
  write(root, 'Cargo.toml', '[package]\nname = "other"\n');
  write(root, 'src/lib.rs', '');
  assert.throws(() => applicationLayout(root), /found 0/);
});

const lines = n => Array.from({length: n}, (_, i) => `const VALUE_${i}: usize = ${i};`).join('\n') + '\n';
function git(root, ...args) {
  return execFileSync('git', ['-C', root, ...args], {encoding: 'utf8'});
}
function trackedFixture() {
  const root = fixture();
  git(root, 'init', '-q');
  write(root, 'src/component.rs', lines(51));
  git(root, 'add', '--', 'src/component.rs');
  git(root, '-c', 'core.hooksPath=/dev/null', '-c', 'user.name=Layout Test',
    '-c', 'user.email=layout@example.invalid', 'commit', '-qm', 'fixture');
  return root;
}
function check(root) {
  const result = spawnSync('bash', [path.join(repository, 'check_file_limits.sh')], {
    cwd: root, encoding: 'utf8', env: {...process.env, FILE_LIMIT_BASE_REF: 'HEAD'},
  });
  fs.appendFileSync(path.join(root, 'checks.log'), result.stdout + result.stderr);
  return result;
}
test('relocation preserves the existing oversize baseline and rejects growth', () => {
  const root = trackedFixture();
  fs.mkdirSync(path.join(root, 'crates/codetether-agent/src'), {recursive: true});
  git(root, 'mv', 'src/component.rs', 'crates/codetether-agent/src/component.rs');
  assert.equal(check(root).status, 0);
  write(root, 'crates/codetether-agent/src/component.rs', lines(52));
  assert.equal(check(root).status, 1);
});
test('a copied file and a new oversized file do not inherit the old baseline', () => {
  const root = trackedFixture();
  write(root, 'crates/codetether-agent/src/component.rs', lines(51));
  assert.equal(check(root).status, 1);
  fs.unlinkSync(path.join(root, 'crates/codetether-agent/src/component.rs'));
  write(root, 'crates/codetether-agent/src/new.rs', lines(51));
  assert.equal(check(root).status, 1);
  write(root, 'crates/codetether-agent/src/new.rs', lines(50));
  assert.equal(check(root).status, 0);
});
