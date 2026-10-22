'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { host } = require('./mock-host.cjs');
const cache = require('../src/cache');
const { create } = require('../src/ui-state');

test('cache settings opt in without carrying credentials', async () => {
  assert.deepEqual(cache.environment({ get: (_, fallback) => fallback }), { CARGO_TERM_COLOR: 'never' });
  const instance = host({ config: { useSccache: true, sccacheExecutable: '/tools/cache path/sccache' } });
  await instance.commands.get('codetether.rust.check')();
  assert.deepEqual(JSON.parse(JSON.stringify(instance.tasks[0].args[4].args[2].env)), {
    CARGO_TERM_COLOR: 'never', RUSTC_WRAPPER: '/tools/cache path/sccache', CARGO_INCREMENTAL: '0'
  });
});
test('cache summaries expose only aggregate counts and backend type', () => {
  const result = cache.summary({ cache_location: 'S3, https://private.invalid/credential',
    stats: { cache_hits: { counts: { Rust: 12 } }, cache_misses: { counts: { Rust: 3 } }, compile_requests: 20 } });
  assert.deepEqual(result, { backend: 'S3-compatible (MinIO/S3)', hits: 12, misses: 3, requests: 20, errors: 0 });
  assert.doesNotMatch(JSON.stringify(result), /credential|private/);
});
test('task state separates workspaces and ignores duplicate termination', () => {
  const state = create();
  const task = workspace => ({ task: { definition: { workspace, type: 'codetether-rust', action: 'check' } } });
  const a = task('a'), b = task('b');
  state.start(a); state.start(b); state.end(a, 1); state.end(a, undefined);
  assert.equal(state.get('a').last.exitCode, 1);
  assert.deepEqual(state.get('b').running, ['check']);
});
test('independent UI tracks starts, failure and launch-only termination', async () => {
  const instance = host({ action: 'check' });
  assert.match(instance.bars[0].text, /Idle/);
  await instance.commands.get('codetether.rust.menu')();
  assert.match(instance.bars[0].text, /sync~spin.*check/);
  instance.finish({ execution: { task: instance.tasks[0] }, exitCode: 1 });
  instance.end(instance.tasks[0]);
  assert.match(instance.bars[0].text, /check failed/);
});
test('cache configuration requires trust and consent', async () => {
  for (const options of [{}, { trusted: false, confirm: 'Enable sccache' }]) {
    const instance = host({ action: 'cache', ...options });
    await instance.commands.get('codetether.rust.menu')();
    assert.equal(instance.updates.length, 0);
  }
  const instance = host({ action: 'cache', confirm: 'Enable sccache' });
  await instance.commands.get('codetether.rust.menu')();
  assert.equal(instance.updates[0].key, 'useSccache');
  assert.equal(instance.updates[0].value, true);
});
test('cache-stat failure hides raw stderr and executable details', async () => {
  await assert.rejects(cache.statistics('/nonexistent/sensitive-path/cache', process.cwd()),
    error => !/sensitive-path|ENOENT/.test(error.message) && /statistics unavailable/.test(error.message));
});