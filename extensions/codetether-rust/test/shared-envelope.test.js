'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const path = require('node:path');
const { pathToFileURL } = require('node:url');
const { decode } = require('../src/shared-envelope');
const root = path.resolve('/fixture');
const uri = pathToFileURL(path.join(root, 'lib.rs')).href;
const diagnostic = { message: 'missing import', code: 'E0432', severity: 1,
  range: { start: { line: 1, character: 4 }, end: { line: 1, character: 5 } } };
const snapshot = () => ({ schema_version: 1, pid: 123, updated_at_ms: 1000,
  workspace_uri: pathToFileURL(root).href, diagnostics: { [uri]: [diagnostic] } });

test('native snapshots carry severity, code, ranges and empty clears', async () => {
  const value = snapshot();
  assert.deepEqual(await decode(value, root, async file => file, 1000, () => true), [[uri, [diagnostic]]]);
  value.diagnostics[uri] = [];
  assert.deepEqual(await decode(value, root, async file => file, 1000, () => true), [[uri, []]]);
});
test('dead, stale, future and other workspace producers cannot publish', async () => {
  assert.deepEqual(await decode(snapshot(), root, async file => file, 1000, () => false), []);
  for (const patch of [{ updated_at_ms: -20000 }, { updated_at_ms: 100000 }, { schema_version: 2 },
    { workspace_uri: pathToFileURL(path.resolve('/other')).href }]) {
    assert.deepEqual(await decode({ ...snapshot(), ...patch }, root, async file => file, 1000, () => true), []);
  }
});
test('symlink escapes and malformed ranges are dropped', async () => {
  const realpath = async file => file.endsWith('lib.rs') ? path.resolve('/outside/lib.rs') : file;
  assert.deepEqual(await decode(snapshot(), root, realpath, 1000, () => true), []);
  const value = snapshot();
  value.diagnostics[uri] = [{ ...diagnostic, range: { start: { line: -1, character: 0 }, end: { line: 0, character: 0 } } }];
  assert.deepEqual(await decode(value, root, async file => file, 1000, () => true), [[uri, []]]);
});