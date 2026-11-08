import test from 'node:test';
import assert from 'node:assert/strict';
import { failure } from './failure.mjs';
import { fixture } from './test-support.mjs';
import { run } from './runner.mjs';
test('failure reporting is allowlisted and does not store raw exception details', () => {
  assert.equal(failure(new Error('Authorization: Bearer private')), 'transport_or_storage_error');
  assert.equal(failure(new Error('Chat API HTTP 429')), 'Chat API HTTP 429');
  assert.equal(failure(new Error('Chat API HTTP 429 token=private')), 'transport_or_storage_error');
});
test('zero usage for nonempty output cannot replenish the total token budget', async t => {
  const f = await fixture(t, [{ text: 'partial', finish: 'length', used: 0 }], { totalOutput: 64 });
  const job = await run(f.root, f.job.id, f.options);
  assert.equal(job.status, 'exhausted'); assert.equal(job.reserved, 64);
  assert.equal(f.calls.length, 1);
});
test('reasoning-only length exhaustion retains budget and records empty-answer category', async t => {
  const f = await fixture(t, [{ text: undefined, finish: 'length' }]);
  await assert.rejects(run(f.root, f.job.id, f.options), /empty_answer/);
  const job = await f.load();
  assert.equal(job.status, 'uncertain'); assert.equal(job.failure, 'empty_answer');
  assert.equal(job.reserved, 64); assert.equal(job.calls, 1); assert.equal(job.parts.length, 0);
});
test('observability callbacks cannot corrupt a committed completion', async t => {
  const f = await fixture(t, [{ text: 'done' }], { steps: ['one'] });
  const job = await run(f.root, f.job.id, { ...f.options, onStep: () => { throw new Error('observer'); } });
  assert.equal(job.status, 'complete'); assert.equal((await f.load()).status, 'complete');
});
