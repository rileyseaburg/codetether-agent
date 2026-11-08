import test from 'node:test';
import assert from 'node:assert/strict';
import { fixture } from './test-support.mjs';
import { run } from './runner.mjs';
import { atomic } from './files.mjs';
test('length continues, summaries bound later context, completed jobs do not replay', async t => {
  const f = await fixture(t, [{ text: 'part A ', finish: 'length' }, { text: 'part B' },
    { text: 'bounded facts' }, { text: 'final answer' }]);
  const job = await run(f.root, f.job.id, f.options);
  assert.equal(job.status, 'complete'); assert.equal(job.calls, 4);
  assert.equal(job.reserved, 12); assert.equal(job.index, 2);
  assert.equal(job.parts.map(part => part.text).join(''), 'part A part Bfinal answer');
  assert.match(f.calls[1].messages[0].content, /part A/);
  assert.match(f.calls[3].messages[0].content, /bounded facts/);
  assert.ok(f.calls.every(call => call.messages[0].content.includes(f.job.objective)));
  assert.doesNotMatch(JSON.stringify(await f.load()), /Bearer fixture/);
  await run(f.root, f.job.id, f.options); assert.equal(f.calls.length, 4);
});
test('missing usage spends reserved budget and cannot create endless continuation', async t => {
  const f = await fixture(t, [{ text: 'partial', finish: 'length', noUsage: true }], { totalOutput: 64 });
  const job = await run(f.root, f.job.id, f.options);
  assert.equal(job.status, 'exhausted'); assert.equal(job.calls, 1); assert.equal(job.reserved, 64);
});
test('repeated continuation stalls instead of appending duplicate output', async t => {
  const f = await fixture(t, [{ text: 'repeat', finish: 'length' }, { text: 'repeat', finish: 'length' }]);
  const job = await run(f.root, f.job.id, f.options);
  assert.equal(job.status, 'stalled'); assert.equal(job.parts.length, 1);
});
test('interrupted request requires explicit resume and never refunds its reservation', async t => {
  const f = await fixture(t, [{ text: 'recovered' }], { steps: ['one'] });
  Object.assign(f.job, { status: 'running', calls: 1, reserved: 64, pending: { id: 'lost', tokens: 64 } });
  await atomic(f.dir, 'state.json', f.job);
  await assert.rejects(run(f.root, f.job.id, f.options), /uncertain/);
  assert.equal(f.calls.length, 0);
  const job = await run(f.root, f.job.id, { ...f.options, recover: true });
  assert.equal(job.status, 'complete'); assert.equal(job.calls, 2); assert.equal(job.reserved, 67);
});
test('endpoint changes cannot redirect persisted private context', async t => {
  const f = await fixture(t, []);
  await assert.rejects(run(f.root, f.job.id, { ...f.options, origin: 'http://127.0.0.1:1' }), /endpoint mismatch/);
  assert.equal(f.calls.length, 0);
});
