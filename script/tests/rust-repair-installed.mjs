// Opt-in installed backend + real configured provider smoke. Only a disposable fixture is edited.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import assert from 'node:assert/strict';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const evidence = fs.mkdtempSync(path.join(root, 'artifacts/rust-repair-installed-'));
const workspace = path.join(evidence, 'workspace');
fs.mkdirSync(path.join(workspace, 'src'), { recursive: true });
const source = 'pub fn answer() -> u32 { "wrong" }\n';
const manifest = '[workspace]\n[package]\nname="repair-live-fixture"\nversion="0.1.0"\nedition="2024"\n';
fs.writeFileSync(path.join(workspace, 'Cargo.toml'), manifest);
fs.writeFileSync(path.join(workspace, 'src/lib.rs'), source);
const request = { schema_version: 1,
  diagnostics: [{ path: 'src/lib.rs', code: 'E0308', message: 'expected u32, found &str; implement answer returning 42 without changing its signature', range: null }],
  files: [{ path: 'src/lib.rs', text: source, editable: true }, { path: 'Cargo.toml', text: manifest, editable: true },
    { path: 'AGENTS.md', text: 'For this fixture only: preserve answer() -> u32; return 42. Do not add dependencies or change Cargo.toml.', editable: false }]
};
const binary = process.env.CODETETHER_TEST_BINARY || 'codetether';
console.log(`Installed repair evidence: ${evidence}`);
const result = spawnSync(binary, ['rust-propose'], { input: JSON.stringify(request), cwd: workspace, encoding: 'utf8', timeout: 150000 });
fs.writeFileSync(path.join(evidence, 'proposal.stdout'), result.stdout || '');
fs.writeFileSync(path.join(evidence, 'proposal.stderr'), result.stderr || '');
fs.writeFileSync(path.join(evidence, 'request.json'), JSON.stringify(request, null, 2));
assert.equal(result.status, 0, result.error?.message || result.stderr);
const proposal = JSON.parse(result.stdout);
assert.equal(proposal.schema_version, 1);
assert.ok(proposal.edits.length > 0);
assert.equal(fs.readFileSync(path.join(workspace, 'src/lib.rs'), 'utf8'), source, 'native proposal must not write source');
let corrected = source;
for (const edit of proposal.edits) {
  assert.equal(edit.path, 'src/lib.rs');
  assert.ok(edit.old_text && corrected.indexOf(edit.old_text) >= 0);
  assert.equal(corrected.indexOf(edit.old_text), corrected.lastIndexOf(edit.old_text));
  corrected = corrected.replace(edit.old_text, edit.new_text);
}
fs.writeFileSync(path.join(workspace, 'src/lib.rs'), corrected);
const check = spawnSync(binary, ['rust', '--cwd', workspace, 'check'], {
  encoding: 'utf8', timeout: 60000, env: { ...process.env, CARGO_TARGET_DIR: path.join(evidence, 'target') }
});
fs.writeFileSync(path.join(evidence, 'check.log'), (check.stdout || '') + (check.stderr || ''));
assert.equal(check.status, 0, check.stderr);
fs.writeFileSync(path.join(evidence, 'result.json'), JSON.stringify({
  provider: 'configured real provider', proposalExit: result.status, proposalDidNotWrite: true,
  editCount: proposal.edits.length, correctedCheckExit: check.status
}, null, 2));