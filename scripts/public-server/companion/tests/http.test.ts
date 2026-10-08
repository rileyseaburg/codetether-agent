import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fixture, request, readUntil, token, input, image } from './fixture.ts';

test('mocked local: pair, isolated auth, real HTTP SSE deltas, reconnect, stop', { timeout: 8000 }, async t => {
  const base = await fixture(t, async ({ delta }): Promise<void> => { delta('Screen '); delta('analysis'); });
  assert.equal((await request(base, '/sessions', input, '')).status, 401);
  const created = await (await request(base, '/sessions', input)).json() as { id: string; code: string };
  const paired = await (await request(base, '/pair', { code: created.code }, '')).json() as { device_token: string };
  const path = `/sessions/${created.id}`, device = paired.device_token;
  assert.equal((await request(base, path, undefined, device, 'DELETE')).status, 401);
  assert.equal((await request(base, `${path}/events`, undefined, device, 'GET')).status, 401);
  const frame = { image: image(), captured_at: new Date().toISOString() };
  assert.equal((await request(base, `${path}/commands`, undefined, token, 'GET')).status, 401);
  const response = await request(base, `${path}/events`, undefined, token, 'GET');
  assert.equal(response.status, 200);
  const reader = response.body!.getReader();
  assert.match(await readUntil(reader, '\n\n'), /snapshot/);
  assert.equal((await request(base, `${path}/frames`, frame, device)).status, 202);
  const events = await readUntil(reader, '"type":"done"');
  assert.match(events, /"type":"delta"/); assert.match(events, /Screen analysis/);
  assert.equal((await request(base, `${path}/frames`, frame, device)).status, 409);
  const replay = await request(base, `${path}/events`, undefined, token, 'GET');
  const replayReader = replay.body!.getReader();
  assert.match(await readUntil(replayReader, '\n\n'), /Screen analysis/);
  assert.equal((await request(base, path, undefined, token, 'DELETE')).status, 200);
  assert.match(await readUntil(reader, 'stopped'), /stopped/);
  assert.equal((await request(base, `${path}/frames`, frame, device)).status, 410);
  await reader.cancel(); await replayReader.cancel();
});
test('mocked local: cross-origin mutation and oversized bodies are denied', async t => {
  const base = await fixture(t, async (): Promise<void> => undefined);
  const response = await fetch(`${base}/sessions`, { method: 'POST', headers: {
    Origin: 'https://untrusted.example', Authorization: `Bearer ${token}`, 'Content-Type': 'application/json'
  }, body: JSON.stringify(input) });
  assert.equal(response.status, 403);
  assert.equal((await request(base, '/sessions', { ...input, prompt: 'x'.repeat(5000) })).status, 413);
});