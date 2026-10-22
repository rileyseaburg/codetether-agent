// Installed-binary smoke: plan only, never execute the returned command.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const evidence = fs.mkdtempSync(path.join(root, 'artifacts/rust-planner-installed-'));
const binary = process.env.CODETETHER_TEST_BINARY || 'codetether';
const version = spawnSync(binary, ['--version'], { encoding: 'utf8' });
assert.equal(version.status, 0);
const input = { action: 'check', cwd: root, __ct_agent_identity_id: 'fixture-agent',
  __ct_session_id: 'fixture-session', __ct_parent_workspace: root,
  __ct_current_model: 'fixture-model', __ct_prior_context_allowed: false };

/** Invoke the installed registry; example: call('accepted', input). */
function call(name, args) {
  const invocation = ['--log-level', 'ERROR', 'mcp', 'call', '--tool', 'rust_build_plan',
    '--arguments', JSON.stringify(args), '--json'];
  const result = spawnSync(binary, invocation, { cwd: root, encoding: 'utf8', timeout: 30000 });
  fs.writeFileSync(path.join(evidence, `${name}.stdout`), result.stdout || '');
  fs.writeFileSync(path.join(evidence, `${name}.stderr`), result.stderr || '');
  assert.equal(result.status, 0, result.error?.message || result.stderr);
  return result;
}

try {
  const accepted = call('accepted', input);
  const plan = JSON.parse(accepted.stdout);
  assert.equal(plan.executed, false);
  assert.equal(plan.executable, 'codetether');
  assert.ok(!accepted.stdout.includes('fixture-agent'));
  assert.ok(!('approval_id' in plan.exec_command));
  const denied = call('denied', { ...input, approval_id: 'forged' });
  assert.match(denied.stderr, /unsupported field: approval_id/);
  assert.equal(denied.stdout.trim(), '');
  fs.writeFileSync(path.join(evidence, 'result.json'), JSON.stringify({
    version: version.stdout.trim(), runtimeMetadataAccepted: true,
    injectedGrantRejected: true, executed: false, exitCode: 0
  }, null, 2));
  console.log(`Installed planner verification: ${evidence}`);
} catch (error) {
  fs.writeFileSync(path.join(evidence, 'failure.txt'), String(error));
  throw error;
}