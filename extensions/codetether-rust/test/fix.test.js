'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { host } = require('./mock-host.cjs');

test('quick fixes attach diagnostics, version and dirty-file guard', () => {
  const diagnostic = { source: 'CodeTether LSP', message: 'missing import', range: {} };
  const instance = host({ diagnostics: [diagnostic] });
  const document = { uri: { toString: () => 'file:///repo/a.rs' }, version: 7, isDirty: false };
  const actions = instance.providers[0].provideCodeActions(document, {}, { diagnostics: [diagnostic] });
  assert.equal(actions.length, 1);
  assert.equal(actions[0].command.command, 'codetether.rust.fixDiagnostic');
  assert.equal(actions[0].command.arguments[0].version, 7);
  document.isDirty = true;
  assert.ok(instance.providers[0].provideCodeActions(document, {}, { diagnostics: [diagnostic] })[0].disabled);
  assert.deepEqual(Array.from(instance.providers[0].provideCodeActions(document, {}, { diagnostics: [{ source: 'other' }] })), []);
});

test('review accepts only unique replacements in explicitly editable snapshots', () => {
  const { replacements } = host().load('fix-review');
  const context = { snapshots: new Map([
    ['Cargo.toml', { text: '[dependencies]\n', editable: true }],
    ['instructions/AGENTS.md', { text: 'policy', editable: false }]
  ]) };
  const proposal = edit => ({ schema_version: 1, summary: 'repair', edits: [edit] });
  const edit = { path: 'Cargo.toml', old_text: '[dependencies]', new_text: '[dependencies]\nasync-trait = "0.1.77"' };
  assert.match(replacements(context, proposal(edit))[0].preview, /async-trait/);
  for (const invalid of [{ ...edit, path: '../Cargo.toml' }, { ...edit, path: '.env' },
    { ...edit, path: 'instructions/AGENTS.md', old_text: 'policy' }, { ...edit, old_text: '' },
    { ...edit, old_text: 'missing' }, { ...edit, approval_id: 'fake' }]) {
    assert.throws(() => replacements(context, proposal(invalid)));
  }
  assert.throws(() => replacements(context, { schema_version: 1, summary: '', edits: [edit, edit] }));
  context.snapshots.set('Cargo.toml', { text: 'same same', editable: true });
  assert.throws(() => replacements(context, proposal({ ...edit, old_text: 'same' })));
});