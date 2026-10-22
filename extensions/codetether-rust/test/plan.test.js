'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { plan, actions } = require('../src/plan');

test('all commands use the first-party CLI in the selected worktree', () => {
  for (const action of actions) {
    const invocation = plan(action, '/repo/work tree', {});
    assert.equal(invocation.executable, 'codetether');
    const toolchain = action.startsWith('cache-') ? [] : ['--toolchain', 'stable'];
    assert.deepEqual(invocation.args, ['rust', '--cwd', '/repo/work tree', ...toolchain, action]);
  }
});
test('run arguments are literal and package selection is explicit', () => {
  const invocation = plan('run', '/repo', { package: 'app', bin: 'app', runArgs: ['$(touch nope)', 'a b'] });
  assert.deepEqual(invocation.args.slice(5), ['--package', 'app', '--bin', 'app', 'run', '--arg=$(touch nope)', '--arg=a b']);
});
test('status omits irrelevant package, binary and runtime arguments', () => {
  assert.deepEqual(plan('status', '/repo', { package: 'app', bin: 'app', runArgs: ['x'] }).args,
    ['rust', '--cwd', '/repo', '--toolchain', 'stable', 'status']);
});
test('invalid commands and missing workspace fail closed', () => {
  assert.throws(() => plan('enable', '/repo', {}));
  assert.throws(() => plan('build', '', {}));
});