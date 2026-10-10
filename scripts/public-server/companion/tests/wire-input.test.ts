import { test } from 'node:test';
import assert from 'node:assert/strict';
import { Registry } from '../registry.ts';
import { sessionInput, pairCode } from '../session-input.ts';
import { captureInput } from '../capture-input.ts';
import { commandSnapshot, requestCapture } from '../commands.ts';
import { wire, wireNow } from './wire-fixture.ts';

test('static/local: Rust wire fixtures match relay session and device contracts', () => {
  const registry = new Registry();
  const input = sessionInput(wire.session_input);
  assert.deepEqual(input, wire.session_input);
  const receipt = registry.create(input, wireNow);
  assert.deepEqual({ ...receipt, id: wire.session_receipt.id, code: wire.session_receipt.code }, wire.session_receipt);
  const session = registry.get(receipt.id, wireNow);
  const paired = registry.pair(pairCode({ code: receipt.code }), wireNow);
  assert.match(paired.device_token, /^[A-Za-z0-9_-]{43}$/);
  assert.deepEqual({ ...paired, id: wire.pair_receipt.id, device_token: wire.pair_receipt.device_token }, wire.pair_receipt);
  assert.equal(pairCode(wire.pair_request), wire.pair_request.code);
  assert.deepEqual(commandSnapshot(session, wireNow), wire.command_idle);
  const requested = requestCapture(session, wire.capture_request, wireNow);
  assert.deepEqual({ ...requested, request_id: wire.request_receipt.request_id }, wire.request_receipt);
  assert.deepEqual(commandSnapshot(session, wireNow), requested);
  session.pending!.id = wire.command_pending.request_id;
  assert.deepEqual(commandSnapshot(session, wireNow), wire.command_pending);
});

test('static/local: Rust capture fixtures obey the real relay parser', () => {
  assert.deepEqual(captureInput(wire.frame_legacy, wireNow), wire.frame_legacy);
  for (const frame of wire.frames) assert.deepEqual(captureInput(frame, wireNow), frame);
  for (const invalid of wire.invalid_frames) assert.throws(() => captureInput(invalid, wireNow));
  assert.throws(() => sessionInput({ ...wire.session_input, interval_seconds: 1 }));
});