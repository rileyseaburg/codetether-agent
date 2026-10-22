'use strict';
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');

function harness(fail = false, cancel = false) {
  let command, release;
  const events = [], errors = [], diagnostic = { source: 'CodeTether Rust', message: 'error', range: { isEqual: () => true } };
  const wait = new Promise(resolve => { release = resolve; });
  const token = { isCancellationRequested: cancel };
  const snapshot = { root: '/fixture', folder: { uri: { toString: () => 'file:///fixture' } }, request: { files: [] } };
  const vscode = {
    CodeActionKind: { QuickFix: 'quickfix' }, ProgressLocation: { Notification: 15 },
    Uri: { parse: value => value },
    languages: { registerCodeActionsProvider() {}, getDiagnostics: () => [diagnostic] },
    commands: { registerCommand: (_, fn) => { command = fn; } },
    workspace: { isTrusted: true, openTextDocument: async () => ({ version: 1, isDirty: false }),
      getConfiguration: () => ({ get: (_, fallback) => fallback }) },
    window: {
      showWarningMessage: async () => 'Generate proposal', showErrorMessage: message => errors.push(message),
      withProgress: async (options, operation) => {
        assert.equal(options.cancellable, true);
        events.push('visible');
        return operation({ report: ({ message }) => events.push(message) }, token);
      }
    }
  };
  const modules = {
    vscode,
    './fix-context': { capture: async () => snapshot, fresh: async () => { events.push('fresh'); } },
    './fix-provider': { propose: async () => { events.push('provider'); await wait; if (fail) throw Error('backend rejected response'); return {}; } },
    './fix-review': { create: () => async () => events.push('review') }
  };
  const module = { exports: {} };
  vm.runInNewContext(fs.readFileSync(require.resolve('../src/fix-actions'), 'utf8'), { module, require: name => modules[name] });
  module.exports.register({ subscriptions: [] }, { isChecking: () => false });
  return { events, errors, release, run: () => command({ uri: 'file:///fixture/a.rs', version: 1, diagnostics: [diagnostic] }) };
}

test('progress is visible before freshness checks and remains during delayed proposals', async () => {
  const h = harness();
  const running = h.run();
  await new Promise(resolve => setImmediate(resolve));
  assert.ok(h.events.indexOf('visible') < h.events.indexOf('fresh'));
  assert.ok(h.events.includes('provider'));
  assert.ok(!h.events.includes('review'));
  h.release(); await running;
  assert.ok(h.events.includes('Opening proposal review…'));
  assert.equal(h.events.at(-1), 'review');
});

test('delayed provider failures surface an error, skip review and release the workspace', async () => {
  const h = harness(true);
  const running = h.run(); h.release(); await running;
  assert.match(h.errors[0], /CodeTether repair: backend rejected response/);
  assert.ok(!h.events.includes('review'));
  await h.run();
  assert.equal(h.events.filter(value => value === 'provider').length, 2);
});

test('cancellation after consent does not invoke provider or review', async () => {
  const h = harness(false, true); await h.run();
  assert.match(h.errors[0], /Repair cancelled/);
  assert.ok(!h.events.includes('provider'));
  assert.ok(!h.events.includes('review'));
});

test('already cancelled native proposal never launches an executable', async () => {
  const { propose } = require('../src/fix-provider');
  await assert.rejects(propose('/nonexistent', {}, '.', '', { isCancellationRequested: true }), /Repair cancelled/);
});