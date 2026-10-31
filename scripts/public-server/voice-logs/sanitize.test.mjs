import test from 'node:test';
import assert from 'node:assert/strict';
import { sanitize } from './sanitize.mjs';
const entry = message => ({ _SYSTEMD_USER_UNIT: 'codetether-audio.service',
  __REALTIME_TIMESTAMP: '1793191200000000', __CURSOR: 'cursor', MESSAGE: message });
test('only safe structured voice metadata reaches NAS', () => {
  const record = sanitize(entry(JSON.stringify({ event: 'voice_bridge', status: 'failed',
    stage: 'authorization', http_status: 401, characters: 100, audio_bytes: 400,
    request_id: '00000000-0000-4000-8000-000000000000',
    authorization: 'Bearer private', script: 'private words', error: 'private response',
    voice: 'private', route: '/private?token=private' })));
  assert.equal(record.http_status, 401); assert.equal(record.stage, 'authorization');
  assert.equal(record.request_id, '00000000-0000-4000-8000-000000000000');
  assert.doesNotMatch(JSON.stringify(record), /private|Bearer/);
});
test('journald byte-array messages are decoded before privacy filtering', () => {
  const input = entry('unused');
  input._SYSTEMD_USER_UNIT = 'codetether-spotlessbinco-thinker.service';
  input.MESSAGE = [...Buffer.from('\u001b[33m WARN codetether_provider::request: timeout token=private')];
  assert.equal(sanitize(input).category, 'timeout');
  assert.doesNotMatch(JSON.stringify(sanitize(input)), /private|token/);
  input.MESSAGE = [999];
  assert.equal(sanitize(input), null);
});
test('agent diagnostics retain category and module, not exception contents', () => {
  const input = entry('\u001b[33m WARN codetether_provider::request: timeout token=private prompt=private');
  input._SYSTEMD_USER_UNIT = 'codetether-spotlessbinco-thinker.service';
  const record = sanitize(input);
  assert.equal(record.category, 'timeout');
  assert.equal(record.module, 'codetether_provider::request');
  assert.doesNotMatch(JSON.stringify(record), /private|token|prompt/);
  input.MESSAGE = 'INFO audit action=GET /api/realtime/session/00000000-0000-4000-8000-000000000000 outcome=Success principal=private';
  assert.equal(sanitize(input).session_id, '00000000-0000-4000-8000-000000000000');
  assert.doesNotMatch(JSON.stringify(sanitize(input)), /private|principal/);
});
test('unrelated or malformed entries are not exported', () => {
  assert.equal(sanitize(entry('not JSON private')), null);
  assert.equal(sanitize(entry('null')), null);
  assert.equal(sanitize(entry('{"event":"private"}')), null);
  const input = entry('{}'); input._SYSTEMD_USER_UNIT = 'unrelated.service';
  assert.equal(sanitize(input), null);
});