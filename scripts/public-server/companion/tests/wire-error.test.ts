import { test } from 'node:test';
import assert from 'node:assert/strict';
import type { SessionReceipt } from '../types.ts';
import { fixture, request, readUntil } from './fixture.ts';
import { wire, decodeEvents, normalizeEvent } from './wire-fixture.ts';

test('mocked local: Rust error event fixture matches redacted inference failure', { timeout: 8000 }, async t => {
  const base = await fixture(t, async (): Promise<void> => { throw new Error('test-only provider detail'); });
  const receipt = await (await request(base, '/sessions', wire.session_input)).json() as SessionReceipt;
  const paired = await (await request(base, '/pair', { code: receipt.code })).json() as { device_token: string };
  const response = await request(base, `/sessions/${receipt.id}/events`, undefined, undefined, 'GET');
  const reader = response.body!.getReader();
  t.after(async () => { await reader.cancel(); });
  const uploaded = await request(base, `/sessions/${receipt.id}/frames`, {
    ...wire.frame_legacy, captured_at: new Date().toISOString()
  }, paired.device_token);
  assert.equal(uploaded.status, 202);
  const events = decodeEvents(await readUntil(reader, '"type":"error"'));
  const error = events.find(event => event.type === 'error')!;
  assert.ok(error);
  assert.deepEqual(normalizeEvent(error, wire.events[4]), wire.events[4]);
});