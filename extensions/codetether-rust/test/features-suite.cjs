'use strict';
const vscode = require('vscode');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const base = require('./vscode-suite.cjs');

async function replace(document, text) {
  const edit = new vscode.WorkspaceEdit();
  edit.replace(document.uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), text);
  assert.ok(await vscode.workspace.applyEdit(edit));
  assert.ok(await document.save());
}
async function waitTask(operation) {
  let timer, listener;
  const ended = new Promise((resolve, reject) => {
    timer = setTimeout(() => reject(Error('Expected Cargo task did not finish')), 90000);
    listener = vscode.tasks.onDidEndTaskProcess(event => {
      if (event.execution.task.definition.type === 'codetether-rust') resolve(event);
    });
  });
  try { await operation(); return await ended; } finally { clearTimeout(timer); listener.dispose(); }
}
async function waitNative(uri, present) {
  for (let attempt = 0; attempt < 60; attempt++) {
    const values = vscode.languages.getDiagnostics(uri).filter(item => item.source === 'CodeTether LSP');
    if (Boolean(values.length) === present) return values;
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw Error(`Native diagnostic state did not become ${present}`);
}

/** Actual editor plumbing with fixture-backed proposal and native snapshot inputs. */
async function run() {
  await base.run();
  const evidence = process.env.CODETETHER_EDITOR_EVIDENCE;
  const folder = vscode.workspace.workspaceFolders[0];
  const uri = vscode.Uri.joinPath(folder.uri, 'src/lib.rs');
  const document = await vscode.workspace.openTextDocument(uri);
  const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
  assert.equal(vscode.extensions.getExtension('rust-lang.rust-analyzer'), undefined);
  const originalPick = vscode.window.showQuickPick;
  vscode.window.showQuickPick = async choices => {
    assert.ok(choices.some(choice => choice.action === 'stats'));
    assert.ok(choices.some(choice => choice.action === 'save'));
    return choices.find(choice => choice.action === 'check');
  };
  const wrapper = path.join(evidence, 'sccache-fixture');
  fs.writeFileSync(wrapper, '#!/bin/sh\nexec "$@"\n', { mode: 0o700 });
  await config.update('sccacheExecutable', wrapper, vscode.ConfigurationTarget.Global);
  await config.update('useSccache', true, vscode.ConfigurationTarget.WorkspaceFolder);
  try {
    const checked = await waitTask(() => vscode.commands.executeCommand('codetether.rust.menu'));
    assert.equal(checked.exitCode, 0);
    assert.equal(checked.execution.task.execution.options.env.RUSTC_WRAPPER, wrapper);
    assert.equal(checked.execution.task.execution.options.env.CARGO_INCREMENTAL, '0');
  } finally {
    vscode.window.showQuickPick = originalPick;
    await config.update('useSccache', false, vscode.ConfigurationTarget.WorkspaceFolder);
  }
  fs.writeFileSync(path.join(evidence, 'ui-cache.json'), JSON.stringify({ validation: 'mocked local',
    rustAnalyzerAbsent: true, menuCheckExit: 0, cacheWrapperForwarded: true, realMinioHits: false }, null, 2));
  await config.update('checkOnSaveDelay', 100, vscode.ConfigurationTarget.Workspace);
  const enableDialog = vscode.window.showWarningMessage;
  vscode.window.showWarningMessage = async () => 'Enable';
  try { await vscode.commands.executeCommand('codetether.rust.enableCheckOnSave'); }
  finally { vscode.window.showWarningMessage = enableDialog; }
  assert.equal(vscode.workspace.getConfiguration('codetether.rust', folder.uri).get('checkOnSave'), true);
  const saved = await waitTask(() => replace(document, 'pub mod missing;\n'));
  assert.equal(saved.exitCode, 1); assert.equal(saved.execution.task.definition.automatic, true);
  const errors = await base.diagnostics(uri, true);
  const actions = await vscode.commands.executeCommand('vscode.executeCodeActionProvider', uri, errors[0].range);
  const fix = actions.find(action => action.command?.command === 'codetether.rust.fixDiagnostic');
  assert.ok(fix, 'diagnostic must expose a native CodeTether quick fix');
  const executable = path.join(evidence, 'proposal-fixture');
  fs.copyFileSync(path.join(__dirname, 'fixtures/proposal-cli.cjs'), executable); fs.chmodSync(executable, 0o700);
  await config.update('executable', executable, vscode.ConfigurationTarget.Global);
  const original = vscode.window.showWarningMessage;
  const approvals = [];
  vscode.window.showWarningMessage = async (...args) => {
    const choice = args.includes('Generate proposal') ? 'Generate proposal' : args.includes('Apply & Check') ? 'Apply & Check' : 'Next file';
    approvals.push(choice); return choice;
  };
  try {
    const repaired = await waitTask(() => vscode.commands.executeCommand(fix.command.command, ...fix.command.arguments));
    assert.equal(repaired.exitCode, 0);
    assert.match(document.getText(), /pub fn repaired/);
    assert.deepEqual(await base.diagnostics(uri, false), []);
    assert.deepEqual(approvals, ['Generate proposal', 'Apply & Check']);
  } finally { vscode.window.showWarningMessage = original; }

  const directory = path.join(evidence, 'native-diagnostics'); fs.mkdirSync(directory);
  await config.update('diagnosticsDirectory', directory, vscode.ConfigurationTarget.Global);
  const envelope = { schema_version: 1, pid: process.pid, workspace_uri: folder.uri.toString(), server: 'fixture', updated_at_ms: Date.now(),
    diagnostics: { [uri.toString()]: [{ message: 'Native fixture diagnostic', severity: 1, code: 'E0432',
      range: { start: { line: 0, character: 0 }, end: { line: 0, character: 1 } } }] } };
  const snapshot = path.join(directory, 'fixture.json');
  fs.writeFileSync(snapshot, JSON.stringify(envelope));
  assert.equal((await waitNative(uri, true))[0].code, 'E0432');
  envelope.diagnostics = {}; envelope.updated_at_ms = Date.now();
  fs.writeFileSync(snapshot, JSON.stringify(envelope));
  assert.deepEqual(await waitNative(uri, false), []);
  fs.writeFileSync(path.join(evidence, 'features.json'), JSON.stringify({
    fixtureBacked: true, checkOnSave: true, diagnosticQuickFix: true,
    reviewedApplyAndCargoCheck: true, nativeSnapshotPublishedAndCleared: true
  }, null, 2));
}
module.exports = { run };