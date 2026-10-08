import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { resolve, join } from 'node:path';

// Authenticated capability check only: creates a fresh empty session, never sends a prompt.
const out = process.argv[2];
if (!out) throw new Error('Usage: probe-model-selection.mjs ARTIFACT_DIRECTORY');
mkdirSync(out, { recursive: true });
const require = createRequire(resolve(process.env.CODETETHER_PROBE_PACKAGE || 'marketing-site/package.json'));
const WebSocket = require('ws');
const token = execFileSync('vault', ['kv', 'get', '-field=token',
  'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const headers = { Authorization: `Bearer ${token}` };
const response = await fetch('https://server.codetether.run/api/session', {
  method: 'POST', headers: { ...headers, 'Content-Type': 'application/json' },
  body: JSON.stringify({ title: 'Voice model capability check', agent: 'build' }),
  signal: AbortSignal.timeout(30000)
});
if (!response.ok) throw new Error(`Session create HTTP ${response.status}`);
const session = await response.json();
const socket = new WebSocket(`wss://server.codetether.run/api/realtime/session/${session.id}`, { headers });
await new Promise((resolveDone, reject) => {
  const timer = setTimeout(() => { socket.terminate(); reject(new Error('Ready frame timed out')); }, 15000);
  socket.on('error', () => { clearTimeout(timer); reject(new Error('Realtime connection failed')); });
  socket.on('message', data => {
    try {
      const frame = JSON.parse(data.toString());
      if (frame.type !== 'ready') return;
      clearTimeout(timer);
      const evidence = { checkedAt: new Date().toISOString(), sessionID: session.id,
        type: frame.type, model_selection: frame.model_selection === true };
      writeFileSync(join(out, 'model-selection-ready.json'), JSON.stringify(evidence, null, 2));
      console.log(JSON.stringify(evidence));
      socket.close();
      if (evidence.model_selection) resolveDone();
      else reject(new Error('Server does not advertise backend model selection'));
    } catch {
      clearTimeout(timer); socket.terminate(); reject(new Error('Invalid ready frame'));
    }
  });
});