// Live relay probe with a synthetic device; never captures or injects keyboard input.
import { writeFileSync } from 'node:fs';
import assert from 'node:assert/strict';
const [output, phase] = process.argv.slice(2);
assert(output && ['before', 'after'].includes(phase));
const token = process.env.CODETETHER_AUTH_TOKEN;
assert(token && token.length >= 32, 'Owner credential is unavailable');
const base = 'https://server.codetether.run/companion';
const evidence = { level: 'live relay, synthetic device', phase, steps: [] };
let id;
async function request(name, path, method, credential, body) {
  const response = await fetch(base + path, { method, redirect: 'error',
    signal: AbortSignal.timeout(20000), headers: { 'Content-Type': 'application/json',
      'Cache-Control': 'no-store', Authorization: `Bearer ${credential}` },
    body: body === undefined ? undefined : JSON.stringify(body) });
  evidence.steps.push({ name, status: response.status });
  return response;
}
try {
  const created = await request('create', '/sessions', 'POST', token,
    { model: 'test/no-inference', prompt: 'Integration check', interval_seconds: 15 });
  assert.equal(created.status, 200);
  const session = await created.json(); id = session.id; evidence.sessionId = id;
  const paired = await request('pair', '/pair', 'POST', token, { code: session.code });
  assert.equal(paired.status, 200);
  const device = (await paired.json()).device_token;
  const path = `/sessions/${id}`;
  assert.equal((await request('commands-before', path + '/commands', 'GET', device)).status, 200);
  const reply = await request('queue-reply', path + '/reply', 'POST', token,
    { text: 'CodeTether harmless keyboard integration check' });
  assert.equal(reply.status, phase === 'before' ? 404 : 202);
  if (phase === 'after') {
    const queued = await reply.json();
    const commands = await request('poll-reply', path + '/commands', 'GET', device);
    assert.equal((await commands.json()).reply.id, queued.reply_id);
    const ack = await request('acknowledge-consumption', path + '/typed', 'POST', device,
      { reply_id: queued.reply_id });
    assert.equal(ack.status, 200); assert.equal((await ack.json()).typed, true);
    const empty = await request('poll-empty', path + '/commands', 'GET', device);
    assert.equal((await empty.json()).reply, undefined);
  }
  assert.equal((await request('commands-still-valid', path + '/commands', 'GET', device)).status, 200);
  evidence.success = true;
} catch (error) { evidence.success = false; evidence.errorType = error.name; process.exitCode = 1; }
finally {
  if (id) await request('stop-test-session', `/sessions/${id}`, 'DELETE', token);
  writeFileSync(output, JSON.stringify(evidence, null, 2) + '\n', { mode: 0o600 });
  console.log(JSON.stringify(evidence));
}