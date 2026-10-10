// Mocked local: real HTTP handlers, synthetic device; no Windows keystrokes.
import test from 'node:test';
import assert from 'node:assert/strict';
import { fixture, input, request, token } from '../../../../scripts/public-server/companion/tests/fixture.ts';
test('mocked local: reply routes, capability separation, repeat polls and consumption ACK', async t => {
  const base = await fixture(t, async () => { throw new Error('Inference must not run'); });
  const session = await (await request(base, '/sessions', input)).json();
  const device = await (await request(base, '/pair', { code: session.code })).json();
  const path = `/sessions/${session.id}`;
  const body = { text: 'Harmless keyboard test' };
  assert.equal((await request(base, path + '/reply', body, device.device_token)).status, 401);
  assert.equal((await request(base, path + '/commands', undefined, token, 'GET')).status, 401);
  const queued = await request(base, path + '/reply', body);
  assert.equal(queued.status, 202);
  const { reply_id } = await queued.json();
  for (let i = 0; i < 2; i++) {
    const poll = await request(base, path + '/commands', undefined, device.device_token, 'GET');
    assert.equal(poll.status, 200);
    assert.deepEqual((await poll.json()).reply, { id: reply_id, text: body.text });
  }
  assert.equal((await request(base, path + '/reply', body)).status, 409);
  assert.equal((await request(base, path + '/typed', { reply_id })).status, 401);
  for (const typed of [true, false]) {
    const ack = await request(base, path + '/typed', { reply_id }, device.device_token);
    assert.equal(ack.status, 200); assert.deepEqual(await ack.json(), { typed });
  }
  const empty = await request(base, path + '/commands', undefined, device.device_token, 'GET');
  assert.equal((await empty.json()).reply, undefined);
  assert.equal((await request(base, path, undefined, token, 'DELETE')).status, 200);
  assert.equal((await request(base, path + '/commands', undefined, device.device_token, 'GET')).status, 410);
});
test('mocked local: pause clears pending typing and prevents new replies', async t => {
  const base = await fixture(t, async () => {});
  const session = await (await request(base, '/sessions', input)).json();
  const device = await (await request(base, '/pair', { code: session.code })).json();
  const path = `/sessions/${session.id}`;
  assert.equal((await request(base, path + '/reply', { text: 'Test' })).status, 202);
  assert.equal((await request(base, path + '/pause', {}, device.device_token)).status, 200);
  const poll = await request(base, path + '/commands', undefined, device.device_token, 'GET');
  assert.equal((await poll.json()).reply, undefined);
  assert.equal((await request(base, path + '/reply', { text: 'Test' })).status, 409);
});
// `typed: true` proves queue consumption only. It is deliberately not asserted as
// insertion into a Windows field; that requires an interactive Windows device.
