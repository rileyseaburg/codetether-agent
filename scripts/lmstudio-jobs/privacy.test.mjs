import test from 'node:test';
import assert from 'node:assert/strict';
import { setTimeout as delay } from 'node:timers/promises';
import { symlink } from 'node:fs/promises';
import { join } from 'node:path';
import { fixture } from './test-support.mjs';
import { run } from './runner.mjs';
import { cancel, lock } from './lock.mjs';
import { directory } from './files.mjs';
import { plan, bounded } from './plan.mjs';
import { tail } from './tail.mjs';
test('cancellation aborts active HTTP and cannot be silently resumed', async t => {
  const f = await fixture(t, [{ hang: true }]);
  const running = run(f.root, f.job.id, f.options);
  const rejected = assert.rejects(running, /cancelled/);
  for (let attempt = 0; attempt < 100 && !f.calls.length; attempt++) await delay(5);
  assert.equal(f.calls.length, 1);
  await cancel(f.dir); await rejected;
  const saved = await f.load(); assert.equal(saved.status, 'cancelled'); assert.equal(saved.reserved, 64);
  await run(f.root, f.job.id, { ...f.options, recover: true }); assert.equal(f.calls.length, 1);
});
test('an existing live writer cannot be recovered or raced', async t => {
  const f = await fixture(t, []), release = await lock(f.dir);
  try {
    await assert.rejects(lock(f.dir), /writer/);
    await assert.rejects(lock(f.dir, true), /still running/);
  } finally { await release(); }
  const second = await lock(f.dir); await second();
});
test('IDs and symlinks cannot escape the private job root', async t => {
  const f = await fixture(t, []);
  await assert.rejects(directory(f.root, '../escape'), /Invalid job ID/);
  const id = '00000000-0000-4000-8000-000000000000';
  await symlink(f.dir, join(f.root, id));
  await assert.rejects(directory(f.root, id), /private/);
});
test('chunking is bounded without losing Unicode or replacing original instructions', () => {
  const document = '界😀text'.repeat(1000);
  const job = plan({ objective: 'fixed requirements', document });
  assert.equal(job.steps.join(''), document);
  assert.ok(job.steps.every(step => Buffer.byteLength(step) <= 6000));
  assert.equal(bounded('😀😀x', 5), '😀'); assert.equal(tail('abc😀x', 5), '😀x');
  assert.throws(() => plan({ objective: 'x', steps: ['x'], model: 'other/model' }));
  assert.throws(() => plan({ objective: 'x', steps: ['x'], maxCalls: 1000 }));
});
