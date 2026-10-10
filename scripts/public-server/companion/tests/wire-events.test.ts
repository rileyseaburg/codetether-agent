import { test } from 'node:test';
import assert from 'node:assert/strict';
import type { SessionReceipt } from '../types.ts';
import { fixture, request, readUntil } from './fixture.ts';
import { wire, decodeEvents, normalizeEvent } from './wire-fixture.ts';

test('mocked local: shared Rust fixtures match HTTP receipts and streamed events', { timeout: 8000 }, async t => {
  const base = await fixture(t, async ({ delta }): Promise<void> => { delta('Visible editor'); });
  const receipt = await (await request(base, '/sessions', wire.session_input)).json() as SessionReceipt;
  const response = await request(base, `/sessions/${receipt.id}/events`, undefined, undefined, 'GET');
  const reader = response.body!.getReader();
  t.after(async () => { await reader.cancel(); });
  const initial = decodeEvents(await readUntil(reader, '"type":"snapshot"'))[0];
  assert.deepEqual(initial, wire.events[0]);
  const paired = await (await request(base, '/pair', { code: receipt.code })).json() as { device_token: string };
  const frame = { ...wire.frames[0], captured_at: new Date().toISOString() };
  const accepted = await request(base, `/sessions/${receipt.id}/frames`, frame, paired.device_token);
  assert.equal(accepted.status, 202);
  assert.deepEqual(await accepted.json(), wire.accepted);
  const events = decodeEvents(await readUntil(reader, '"type":"done"'));
  for (const expected of wire.events.slice(1, 4)) {
    const actual = events.find(event => event.type === expected.type)!;
    assert.ok(actual);
    assert.deepEqual(normalizeEvent(actual, expected), expected);
  }
  const paused = await request(base, `/sessions/${receipt.id}/pause`, {}, paired.device_token);
  assert.deepEqual(await paused.json(), wire.paused);
  const stopped = await request(base, `/sessions/${receipt.id}`, undefined, undefined, 'DELETE');
  assert.deepEqual(await stopped.json(), wire.stopped);
  const terminal = decodeEvents(await readUntil(reader, '"type":"stopped"')).find(event => event.type === 'stopped')!;
  assert.ok(terminal);
  assert.deepEqual(normalizeEvent(terminal, wire.events[5]), wire.events[5]);
  const revoked = await request(base, `/sessions/${receipt.id}/frames`, frame, paired.device_token);
  assert.equal(revoked.status, 410);
  assert.deepEqual(await revoked.json(), wire.error);
});