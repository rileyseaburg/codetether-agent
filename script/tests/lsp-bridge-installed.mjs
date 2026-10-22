// Opt-in installed-native smoke; creates only an isolated Rust/LSP fixture.
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { once } from 'node:events';
import assert from 'node:assert/strict';
import envelope from '../../extensions/codetether-rust/src/shared-envelope.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const evidence = await fs.mkdtemp(path.join(root, 'artifacts/lsp-bridge-installed-'));
const workspace = path.join(evidence, 'workspace');
const snapshots = path.join(evidence, 'snapshots');
await fs.mkdir(path.join(workspace, 'src'), { recursive: true });
await fs.mkdir(snapshots, { mode: 0o700 });
await fs.writeFile(path.join(workspace, 'Cargo.toml'), '[workspace]\n[package]\nname="bridge-fixture"\nversion="0.1.0"\nedition="2024"\n');
const source = path.join(workspace, 'src/lib.rs');
await fs.writeFile(source, 'pub fn broken( {\n');
console.log(`Installed LSP bridge evidence: ${evidence}`);
const child = spawn(process.env.CODETETHER_TEST_BINARY || 'codetether', ['mcp', 'serve'], {
  cwd: workspace, stdio: ['pipe', 'pipe', 'pipe'],
  env: { ...process.env, CODETETHER_DIAGNOSTICS_DIR: snapshots,
    CARGO_TARGET_DIR: path.join(evidence, 'target') }
});
let stderr = '', nextId = 0;
const pending = new Map();
child.stderr.setEncoding('utf8');
child.stderr.on('data', chunk => { stderr += chunk; });
const lines = createInterface({ input: child.stdout });
lines.on('line', line => {
  let reply;
  try { reply = JSON.parse(line); } catch { return; }
  const receiver = pending.get(reply.id);
  if (receiver) { pending.delete(reply.id); receiver(reply); }
});
async function rpc(method, params) {
  const id = ++nextId;
  let timer;
  try {
    const answer = new Promise((resolve, reject) => {
      pending.set(id, resolve);
      timer = setTimeout(() => reject(new Error(`MCP timeout: ${method}`)), 90000);
    });
    child.stdin.write(`${JSON.stringify({ jsonrpc: '2.0', id, method, params })}\n`);
    const reply = await answer;
    assert.ok(!reply.error, JSON.stringify(reply.error));
    assert.ok(!reply.result?.isError, JSON.stringify(reply.result));
    return reply;
  } finally { clearTimeout(timer); pending.delete(id); }
}
async function waitSnapshot(hasDiagnostics) {
  const uri = pathToFileURL(source).href;
  for (let attempt = 0; attempt < 60; attempt++) {
    for (const name of await fs.readdir(snapshots)) {
      if (!name.endsWith('.json')) continue;
      const snapshot = JSON.parse(await fs.readFile(path.join(snapshots, name), 'utf8'));
      const decoded = await envelope.decode(snapshot, await fs.realpath(workspace), fs.realpath);
      if (decoded.some(([file, items]) => file === uri && (items.length > 0) === hasDiagnostics)) {
        await fs.writeFile(path.join(evidence, hasDiagnostics ? 'published.json' : 'cleared.json'), JSON.stringify(snapshot, null, 2));
        return;
      }
    }
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
  throw new Error(`No ${hasDiagnostics ? 'nonempty' : 'empty'} native snapshot accepted by editor consumer`);
}
try {
  await rpc('initialize', { protocolVersion: '2024-11-05', capabilities: {}, clientInfo: { name: 'bridge-smoke', version: '1' } });
  child.stdin.write(`${JSON.stringify({ jsonrpc: '2.0', method: 'notifications/initialized' })}\n`);
  const args = { name: 'lsp', arguments: { action: 'diagnostics', file_path: source } };
  await fs.writeFile(path.join(evidence, 'diagnostics-reply.json'), JSON.stringify(await rpc('tools/call', args), null, 2));
  await waitSnapshot(true);
  await fs.writeFile(source, 'pub fn answer() -> u32 { 42 }\n');
  await fs.writeFile(path.join(evidence, 'clear-reply.json'), JSON.stringify(await rpc('tools/call', args), null, 2));
  await waitSnapshot(false);
  await fs.writeFile(path.join(evidence, 'result.json'), JSON.stringify({ nativePublisher: true, editorConsumerAccepted: true, correctedDiagnosticsCleared: true }, null, 2));
} finally {
  lines.close();
  child.stdin.end();
  const exited = once(child, 'exit');
  const timer = setTimeout(() => child.kill('SIGTERM'), 5000);
  await exited;
  clearTimeout(timer);
  await fs.writeFile(path.join(evidence, 'native.stderr'), stderr);
}