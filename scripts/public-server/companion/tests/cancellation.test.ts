import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fixture, request, readUntil, token, input, image } from './fixture.ts';
import { captureInput } from '../capture-input.ts';

test('mocked local: closing iPhone preserves capture; device pause aborts analysis', { timeout: 8000 }, async t => {
  let aborted = false;
  const base = await fixture(t, async ({ signal }): Promise<void> => {
    await new Promise<void>(resolve => signal.addEventListener('abort', () => { aborted = true; resolve(); }, { once: true }));
  });
  const receipt = await (await request(base, '/sessions', input)).json() as { id: string; code: string };
  const pair = await (await request(base, '/pair', { code: receipt.code }, '')).json() as { device_token: string };
  const path = `/sessions/${receipt.id}`;
  const response = await request(base, `${path}/events`, undefined, token, 'GET');
  const reader = response.body!.getReader(); await readUntil(reader, '\n\n');
  assert.equal((await request(base, `${path}/frames`, { image: image(), captured_at: new Date().toISOString() }, pair.device_token)).status, 202);
  await reader.cancel();
  await new Promise(resolve => setTimeout(resolve, 30));
  assert.equal(aborted, false);
  assert.equal((await request(base, `${path}/pause`, {}, pair.device_token)).status, 200);
  for (let i = 0; i < 100 && !aborted; i++) await new Promise(resolve => setTimeout(resolve, 10));
  assert.equal(aborted, true);
  await request(base, path, undefined, token, 'DELETE');
});
test('static/local: reject invalid, stale and oversized screenshot headers', () => {
  const now = Date.now(), valid = { image: image(), captured_at: new Date(now).toISOString() };
  assert.deepEqual(captureInput(valid, now), valid);
  const huge = Buffer.from(image(), 'base64'); huge.writeUInt16BE(5000, 7);
  for (const invalid of [
    { ...valid, image: 'AAAA' }, { ...valid, image: huge.toString('base64') },
    { ...valid, captured_at: new Date(now - 400000).toISOString() },
    { ...valid, image: 'A'.repeat(710000) }
  ]) assert.throws(() => captureInput(invalid, now));
});