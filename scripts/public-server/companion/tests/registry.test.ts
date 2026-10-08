import { test } from 'node:test';
import assert from 'node:assert/strict';
import { Registry } from '../registry.ts';
import { stop } from '../events.ts';
import { sessionInput } from '../session-input.ts';

const input = { model: 'provider/vision', prompt: 'Describe the screen', interval_seconds: 30 };
test('mocked local: one-use pairing, TTL, scoped capability and revocation', () => {
  const registry = new Registry(), now = 1000000;
  const receipt = registry.create(input, now);
  assert.match(receipt.code, /^[A-F0-9]{12}$/);
  const pair = registry.pair(receipt.code, now);
  assert.equal(pair.id, receipt.id);
  assert.notEqual(registry.get(receipt.id, now).deviceHash, pair.device_token);
  assert.throws(() => registry.pair(receipt.code, now), /invalid or expired/);
  stop(registry.get(receipt.id, now));
  assert.throws(() => registry.get(receipt.id, now), /ended/);
  const expired = registry.create(input, now);
  assert.throws(() => registry.pair(expired.code, now + 300001), /expired/);
  registry.sweep(now + 3600001); assert.equal(registry.sessions.size, 0);
});
test('mocked local: session capacity and pairing rate are bounded', () => {
  const registry = new Registry();
  for (let i = 0; i < 4; i++) registry.create(input);
  assert.throws(() => registry.create(input), /Stop an existing/);
  for (let i = 0; i < 30; i++) assert.throws(() => registry.pair('000000000000'), /invalid or expired/);
  assert.throws(() => registry.pair('000000000000'), /Too many/);
});
test('static/local: strict model, instructions and minimum interval', () => {
  assert.deepEqual(sessionInput(input), input);
  for (const invalid of [{ ...input, interval_seconds: 1 }, { ...input, prompt: '' }, { ...input, model: 'vision' }]) {
    assert.throws(() => sessionInput(invalid));
  }
});