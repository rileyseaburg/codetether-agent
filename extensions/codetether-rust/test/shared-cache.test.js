'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { host } = require('./mock-host.cjs');
const { plan } = require('../src/plan');

test('cache actions use native backend and repository-owned selectors/toolchain', () => {
  for (const action of ['cache-build', 'cache-publish']) {
    const value = plan(action, '/repo/work tree', { toolchain: 'nightly', package: 'app', bin: 'app', runArgs: ['--force'] });
    assert.deepEqual(value.args, ['rust', '--cwd', '/repo/work tree', action]);
    assert.equal(value.executable, 'codetether');
  }
});

test('cache commands require trust, script, consent and saved files', async () => {
  for (const action of ['cache-build', 'cache-publish']) {
    for (const options of [{}, { trusted: false }, { missing: true, confirm: 'Continue' }, { save: false, confirm: 'Continue' }]) {
      const instance = host(options);
      await instance.commands.get(`codetether.rust.${action}`)();
      assert.equal(instance.tasks.length, 0);
    }
    const instance = host({ confirm: 'Continue', config: { useSccache: true, sccacheExecutable: '/wrong-wrapper' } });
    await instance.commands.get(`codetether.rust.${action}`)();
    assert.equal(instance.tasks.length, 1);
    const execution = instance.tasks[0].args[4].args;
    assert.equal(execution[1].at(-1), action);
    assert.equal(execution[2].env.RUSTC_WRAPPER, undefined);
    instance.finish({ execution: { task: instance.tasks[0] }, exitCode: 0 });
    assert.match(instance.bars[0].text, /see publication log/);
  }
});

test('shared cache actions cannot run automatically', async () => {
  const instance = host({ confirm: 'Continue' });
  const runner = instance.load('tasks').create({ subscriptions: [] });
  await assert.rejects(runner.run('cache-build', instance.folder, true), /explicitly requested/);
  assert.equal(instance.tasks.length, 0);
});

// These mocks deliberately do not claim remote cache publication or reuse.