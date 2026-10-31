import test from 'node:test';
import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import { fixture } from './test-support.mjs';
import { requestTrace } from './trace.mjs';
test('success correlates request ID and logs timing, never text or credentials', async t => {
  const records = [], secret = 'private spoken words';
  const { url } = await fixture(t, record => records.push(record));
  const response = await fetch(`${url}/tts/speak?secret=hidden`, { method: 'POST',
    headers: { Authorization: 'Bearer fixture' }, body: JSON.stringify({ script: secret }) });
  await response.arrayBuffer();
  assert.equal(response.status, 200);
  assert.ok(records.every(record => record.request_id === response.headers.get('x-request-id')));
  assert.equal(records.at(-1).status, 'completed');
  assert.equal(records.at(-1).characters, secret.length);
  assert.equal(records.at(-1).upstream_status, 200);
  assert.ok(records.at(-1).elapsed_ms >= 0);
  assert.doesNotMatch(JSON.stringify(records), /private spoken|fixture|hidden|Authorization/);
});
test('authorization and upstream failures retain distinct stages and statuses', async t => {
  const records = [];
  const { url } = await fixture(t, record => records.push(record), 503);
  assert.equal((await fetch(`${url}/tts/health`)).status, 401);
  assert.equal(records.at(-1).stage, 'authorization');
  const response = await fetch(`${url}/tts/speak`, { method: 'POST',
    headers: { Authorization: 'Bearer fixture' }, body: JSON.stringify({ script: 'hello' }) });
  await response.text();
  assert.equal(response.status, 502);
  assert.equal(records.at(-1).upstream_status, 503);
  assert.equal(records.at(-1).stage, 'upstream_response');
  assert.equal(records.at(-1).error_type, 'HTTPError');
});
test('disconnect emits one terminal event and logger failure cannot break playback', () => {
  const response = new EventEmitter(); response.setHeader = () => {}; response.statusCode = 200;
  const records = [];
  requestTrace({ method: 'POST', url: '/tts/speak' }, response, record => records.push(record));
  response.emit('close'); response.emit('close');
  assert.equal(records.filter(record => record.status === 'client_disconnected').length, 1);
  assert.doesNotThrow(() => {
    const trace = requestTrace({ method: 'GET', url: '/tts/health' }, response, () => { throw new Error('offline'); });
    trace.mark('synthesis'); trace.fail(new Error('credential must not be logged'));
    response.emit('finish');
  });
});