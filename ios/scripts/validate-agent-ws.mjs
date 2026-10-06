import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { resolve, join } from 'node:path';
import { collectTools } from './collect-agent-tools.mjs';
const require = createRequire(resolve('marketing-site/package.json'));
const WebSocket = require('ws');
const out = process.argv[2];
mkdirSync(out, { recursive: true });
const token = execFileSync('vault', ['kv', 'get', '-field=token', 'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const headers = { Authorization: `Bearer ${token}` };
const response = await fetch('https://server.codetether.run/api/session', {
  method: 'POST', headers: { ...headers, 'Content-Type': 'application/json' },
  body: JSON.stringify({ title: 'iPhone native agent transport validation', agent: 'build' })
});
if (!response.ok) throw new Error(`Session create HTTP ${response.status}`);
const session = await response.json();
writeFileSync(join(out, 'session-id.txt'), session.id);
console.log(JSON.stringify({ sessionID: session.id }));
const events = [];
const socket = new WebSocket(`wss://server.codetether.run/api/realtime/session/${session.id}`, { headers });
const prompt = process.argv[3] || 'Use exec_command to execute printf WS_TOOL_OK. Do not read or write files, prior memory or history. Report the real tool result.';
await new Promise((resolveDone, reject) => {
  const timer = setTimeout(() => { socket.send('{"type":"cancel"}'); socket.close(); reject(new Error('Agent timed out')); }, Number(process.env.AGENT_TIMEOUT_MS || 240000));
  socket.on('open', () => socket.send(JSON.stringify({ type: 'prompt', message: prompt })));
  socket.on('error', error => { clearTimeout(timer); reject(error); });
  socket.on('message', data => {
    const frame = JSON.parse(data.toString());
    if (frame.event) {
      events.push(frame.event.kind);
      writeFileSync(join(out, 'event-kinds.json'), JSON.stringify(events));
      if (frame.event.kind.startsWith('tool.')) console.log(JSON.stringify({ event: frame.event.kind, tool: frame.event.payload?.name }));
    }
    if (frame.type === 'error') { clearTimeout(timer); socket.close(); reject(new Error(frame.message)); }
    if (frame.type === 'result') {
      clearTimeout(timer);
      writeFileSync(join(out, 'result.json'), JSON.stringify({ sessionID: session.id, events, result: frame.result }, null, 2));
      console.log(JSON.stringify({ sessionID: session.id, result: frame.result, eventKinds: [...new Set(events)] }));
      socket.close(); resolveDone();
    }
  });
});
await collectTools(session.id, headers, out);
