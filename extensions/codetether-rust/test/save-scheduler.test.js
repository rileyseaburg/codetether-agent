'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { createScheduler } = require('../src/save-scheduler');

test('rapid saves coalesce and busy workspace gets one subsequent check', async () => {
  const callbacks = new Map(), calls = [];
  let id = 0, running = false;
  const folder = { uri: { toString: () => 'workspace' } };
  const scheduler = createScheduler({ delay: () => 100, busy: () => running,
    run: async value => { calls.push(value); running = true; }, report: assert.fail,
    timers: { setTimeout: callback => { callbacks.set(++id, callback); return id; }, clearTimeout: key => callbacks.delete(key) } });
  scheduler.schedule(folder); scheduler.schedule(folder); scheduler.schedule(folder);
  assert.equal(callbacks.size, 1);
  await [...callbacks.values()][0](); callbacks.clear();
  assert.equal(calls.length, 1);
  scheduler.schedule(folder); scheduler.schedule(folder);
  await [...callbacks.values()][0](); callbacks.clear();
  assert.equal(calls.length, 1);
  running = false; scheduler.idle('workspace');
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(calls.length, 2);
  scheduler.dispose(); scheduler.schedule(folder);
  assert.equal(callbacks.size, 0);
});

test('removing a workspace cancels its pending save check', () => {
  const callbacks = new Map();
  const scheduler = createScheduler({ delay: () => 100, busy: () => false,
    run: assert.fail, report: assert.fail,
    timers: { setTimeout: callback => { callbacks.set(1, callback); return 1; }, clearTimeout: key => callbacks.delete(key) } });
  scheduler.schedule({ uri: { toString: () => 'removed' } });
  scheduler.cancel('removed');
  assert.equal(callbacks.size, 0);
});