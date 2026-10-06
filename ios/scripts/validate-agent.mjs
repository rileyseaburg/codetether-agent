import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const out = process.argv[2];
mkdirSync(out, { recursive: true });
const token = execFileSync('vault', ['kv', 'get', '-field=token', 'secret/codetether/endpoints/public-server'], { encoding: 'utf8' }).trim();
const base = 'https://server.codetether.run';
async function call(path, body) {
  const response = await fetch(base + path, { method: body ? 'POST' : 'GET', redirect: 'error',
    headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined, signal: AbortSignal.timeout(180000) });
  const text = (await response.text()).replaceAll(token, '[REDACTED]');
  writeFileSync(join(out, `${body ? 'post' : 'get'}-${path.split('/').pop()}.json`), text);
  console.log(JSON.stringify({ path, status: response.status }));
  if (!response.ok) throw new Error(`Agent endpoint returned ${response.status}: ${text.slice(0, 500)}`);
  return JSON.parse(text);
}
const session = await call('/api/session', { title: 'iPhone agent tool API validation', agent: 'build' });
console.log(JSON.stringify({ sessionID: session.id, model: session.metadata?.model }));
const result = await call(`/api/session/${session.id}/prompt`, {
  message: 'Use the bash tool to execute printf CODETETHER_TOOL_OK. Do not read or write files, memory or prior sessions. Report the actual tool result.'
});
console.log(JSON.stringify(result));
await call(`/api/session/${session.id}`);
