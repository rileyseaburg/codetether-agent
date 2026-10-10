// Mocked local: synthetic image/analyzer exercise fresh-frame authorization.
import test from 'node:test';
import assert from 'node:assert/strict';
import { setTimeout } from 'node:timers/promises';
import { Registry } from '../../../../scripts/public-server/companion/registry.ts';
import { capture, pause } from '../../../../scripts/public-server/companion/capture.ts';
import { commandSnapshot, requestCapture } from '../../../../scripts/public-server/companion/commands.ts';
import { image, input } from '../../../../scripts/public-server/companion/tests/fixture.ts';
const output = '```windows-reply\n{"text":"Harmless test","target":"Test field"}\n```';
for (const mode of ['requested', 'ordinary', 'periodic', 'failed', 'paused', 'malformed']) {
  test(`mocked local: model typing gate ${mode}`, async () => {
    const registry = new Registry();
    const receipt = registry.create(input);
    registry.pair(receipt.code);
    const session = registry.get(receipt.id);
    const request = mode === 'periodic' ? undefined : requestCapture(session,
      { question: mode === 'ordinary' ? 'Describe this screen' : 'Type Harmless test in the field' });
    const frame = { image: image(), captured_at: new Date().toISOString(),
      trigger: 'manual' as const, ...(request ? { request_id: request.request_id } : {}) };
    capture(session, frame, async context => {
      context.delta(mode === 'malformed' ? 'No valid keyboard output' : output);
      if (mode === 'failed') throw new Error('Synthetic failure');
      if (mode === 'paused') pause(session);
    });
    for (let i = 0; i < 100 && session.controller; i++) await setTimeout(5);
    assert.equal(session.controller, undefined);
    const commands = commandSnapshot(session);
    if (mode === 'requested') {
      assert.equal(commands.reply?.text, 'Harmless test');
      assert.equal(commandSnapshot(session).reply?.id, commands.reply?.id);
      assert.match(session.text, /Typing queued/);
    } else assert.equal(commands.reply, undefined);
  });
}
test('mocked local: expired pending reply cannot be replayed', () => {
  const registry = new Registry();
  const receipt = registry.create(input); registry.pair(receipt.code);
  const session = registry.get(receipt.id);
  session.reply = { id: receipt.id, text: 'Test', created: Date.now() - 60000 };
  assert.equal(commandSnapshot(session).reply, undefined);
});
test('mocked local: unmatched fresh frame cannot authorize typing', () => {
  const registry = new Registry();
  const receipt = registry.create(input); registry.pair(receipt.code);
  const session = registry.get(receipt.id); requestCapture(session, { question: 'Type test' });
  assert.throws(() => capture(session, { image: image(), captured_at: new Date().toISOString() }, async () => {}));
  assert.equal(session.reply, undefined);
});