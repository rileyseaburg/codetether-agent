'use strict';
const vscode = require('vscode');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

/** Execute one real extension task; example: await check() returns its exit code. */
async function check() {
  let subscription, timer;
  const ended = new Promise((resolve, reject) => {
    timer = setTimeout(() => reject(Error('Check task timed out')), 90000);
    subscription = vscode.tasks.onDidEndTaskProcess(event => {
      if (event.execution.task.definition.type === 'codetether-rust') resolve(event.exitCode);
    });
  });
  try { await vscode.commands.executeCommand('codetether.rust.check'); return await ended; }
  finally { subscription.dispose(); clearTimeout(timer); }
}

/** Wait for task-owned diagnostic publication; example: await diagnostics(uri, true). */
async function diagnostics(uri, present) {
  for (let attempt = 0; attempt < 100; attempt++) {
    const items = vscode.languages.getDiagnostics(uri).filter(item => item.source === 'CodeTether Rust');
    if ((items.length > 0) === present) return items;
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  throw Error(`Expected diagnostics present=${present}; got ${JSON.stringify(vscode.languages.getDiagnostics(uri))}`);
}

/** Actual VS Code extension test entry; run by test-electron, not the Node mocks. */
async function run() {
  const evidence = process.env.CODETETHER_EDITOR_EVIDENCE;
  const extension = vscode.extensions.getExtension('codetether.codetether-rust');
  assert.ok(extension);
  await extension.activate();
  const uri = vscode.Uri.joinPath(vscode.workspace.workspaceFolders[0].uri, 'src/lib.rs');
  await vscode.window.showTextDocument(await vscode.workspace.openTextDocument(uri));
  assert.equal(await check(), 1);
  const problems = await diagnostics(uri, true);
  assert.ok(problems.some(item => item.code === 'E0583' && item.range.start.line === 0));
  fs.writeFileSync(path.join(evidence, 'diagnostics.json'), JSON.stringify(problems, null, 2));
  const document = await vscode.workspace.openTextDocument(uri);
  const edit = new vscode.WorkspaceEdit();
  edit.replace(uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), 'pub fn good() -> u8 { 42 }\n');
  await vscode.workspace.applyEdit(edit);
  await document.save();
  assert.equal(await check(), 0);
  assert.deepEqual(await diagnostics(uri, false), []);
  fs.writeFileSync(path.join(evidence, 'result.json'), JSON.stringify({
    errorPublished: true, errorCode: 'E0583', openDocument: true,
    correctedCheckExit: 0, staleDiagnosticsCleared: true,
    vscodeVersion: vscode.version, extensionVersion: extension.packageJSON.version
  }, null, 2));
}
module.exports = { run, check, diagnostics };