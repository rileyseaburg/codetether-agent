import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

// Probe a fresh empty session only: never read a user's prior conversations or run a model.
const out = process.argv[2];
if (!out) throw new Error('Usage: probe-session-fork.mjs EVIDENCE_DIRECTORY');
mkdirSync(out, { recursive: true });
const token = execFileSync('vault', ['kv', 'get', '-field=token',
  'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const headers = { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' };
const origin = 'https://server.codetether.run';
const created = await fetch(`${origin}/api/session`, { method: 'POST', headers,
  body: JSON.stringify({ title: 'Edit endpoint capability check', agent: 'build' }),
  signal: AbortSignal.timeout(30000) });
if (!created.ok) throw new Error(`Session creation HTTP ${created.status}`);
const session = await created.json();
const response = await fetch(`${origin}/api/session/${session.id}/fork`, { method: 'POST', headers,
  body: JSON.stringify({ before_message: 0, expected_text: 'capability-probe' }),
  signal: AbortSignal.timeout(30000) });
const evidence = { checkedAt: new Date().toISOString(), sessionID: session.id,
  url: `${origin}/api/session/${session.id}/fork`, status: response.status, supported: response.status === 409 };
writeFileSync(join(out, 'fork-capability.json'), JSON.stringify(evidence, null, 2));
console.log(JSON.stringify(evidence));
if (!evidence.supported) process.exitCode = 1;