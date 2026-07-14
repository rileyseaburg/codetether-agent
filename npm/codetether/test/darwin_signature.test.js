const test = require('node:test');
const assert = require('node:assert/strict');
const { preserveOrAdhocSign } = require('../lib/darwin_signature');

test('preserves a valid Developer ID signature', () => {
  const calls = [];
  const run = (command, args) => {
    calls.push({ command, args });
    return { status: 0 };
  };

  preserveOrAdhocSign('/tmp/codetether', run);

  assert.deepEqual(calls, [{
    command: 'codesign',
    args: ['--verify', '--strict', '/tmp/codetether'],
  }]);
});

test('ad-hoc signs an unsigned legacy binary', () => {
  const calls = [];
  const run = (command, args) => {
    calls.push({ command, args });
    return { status: calls.length === 1 ? 1 : 0 };
  };

  preserveOrAdhocSign('/tmp/codetether', run);

  assert.equal(calls.length, 2);
  assert.deepEqual(calls[1].args, ['--force', '--sign', '-', '/tmp/codetether']);
});

test('reports an ad-hoc signing failure', () => {
  const run = () => ({ status: 1 });
  assert.throws(
    () => preserveOrAdhocSign('/tmp/codetether', run),
    /Unable to apply an ad-hoc signature/,
  );
});
