'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { host } = require('./mock-host.cjs');

test('trusted workspace uses process execution with literal cwd and Rust matcher', async () => {
  const instance = host();
  await instance.commands.get('codetether.rust.build')();
  assert.equal(instance.tasks.length, 1);
  const task = instance.tasks[0];
  assert.equal(task.args[4].args[0], 'codetether');
  assert.equal(task.args[4].args[2].cwd, '/repo/work tree');
  assert.equal(task.args[5][0], '$codetether-rustc');
  assert.equal(task.args[4].args[2].env.CARGO_TERM_COLOR, 'never');
});
test('untrusted, missing manifest, cancelled save and workspace selection launch nothing', async () => {
  for (const options of [{ trusted: false }, { missing: true }, { save: false }, { multi: true, cancel: true }]) {
    const instance = host(options);
    await instance.commands.get('codetether.rust.build')();
    assert.equal(instance.tasks.length, 0);
  }
});
test('stopping the shared daemon requires explicit confirmation', async () => {
  const instance = host();
  await instance.commands.get('codetether.rust.stop')();
  assert.equal(instance.tasks.length, 0);
});

test('failed owned tasks reveal Problems; successful and unrelated tasks do not', async () => {
  const instance = host();
  await instance.commands.get('codetether.rust.check')();
  instance.finish({ execution: { task: instance.tasks[0] }, exitCode: 1 });
  assert.deepEqual(instance.invoked, ['workbench.actions.view.problems']);
  assert.match(instance.errors[0], /exited 1/);
  instance.finish({ execution: { task: instance.tasks[0] }, exitCode: 0 });
  instance.finish({ execution: { task: { definition: { type: 'other' } } }, exitCode: 1 });
  assert.equal(instance.invoked.length, 1);
});