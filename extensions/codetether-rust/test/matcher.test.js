'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const manifest = require('../package.json');
const matcher = manifest.contributes.problemMatchers[0];

test('diagnostic matcher is extension-owned and includes open documents', () => {
  assert.equal(matcher.name, 'codetether-rustc');
  assert.equal(matcher.owner, 'codetether-rust');
  assert.equal(matcher.source, 'CodeTether Rust');
  assert.equal(matcher.applyTo, 'allDocuments');
  assert.deepEqual(matcher.fileLocation, ['autoDetect', '${workspaceFolder}']);
});
test('normal compiler diagnostics produce file, severity, code and location', () => {
  const [header, location] = matcher.pattern.map(item => new RegExp(item.regexp));
  for (const file of ['crates/effects/src/lib.rs', '/a worktree/src/lib.rs', 'C:\\repo\\src\\lib.rs']) {
    const message = header.exec('error[E0583]: file not found for module `authoring`');
    const source = location.exec(`  --> ${file}:17:1`);
    assert.equal(message[1], 'error');
    assert.equal(message[2], 'E0583');
    assert.equal(source[1], file);
    assert.equal(source[2], '17');
    assert.equal(source[3], '1');
  }
  assert.equal(header.exec('warning: unused import')[1], 'warning');
});