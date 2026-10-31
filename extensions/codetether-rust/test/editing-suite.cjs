'use strict';
const vscode = require('vscode');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

async function eventually(operation, predicate, label) {
  const end = Date.now() + 90000;
  let last;
  while (Date.now() < end) {
    last = await operation();
    if (predicate(last)) return last;
    await new Promise(resolve => setTimeout(resolve, 250));
  }
  throw Error(`Editing timeout: ${label}; last=${JSON.stringify(last)}`);
}
async function replace(document, text) {
  const edit = new vscode.WorkspaceEdit();
  edit.replace(document.uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), text);
  assert.ok(await vscode.workspace.applyEdit(edit));
}

/** Exercise actual VS Code providers against real rust-analyzer, on unsaved buffers. */
async function run() {
  const folder = vscode.workspace.workspaceFolders[0];
  const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
  await config.update('checkOnSave', false, vscode.ConfigurationTarget.WorkspaceFolder);
  await config.update('sharedDiagnostics', false, vscode.ConfigurationTarget.WorkspaceFolder);
  const document = await vscode.workspace.openTextDocument(vscode.Uri.joinPath(folder.uri, 'src/lib.rs'));
  await vscode.window.showTextDocument(document);
  await vscode.extensions.getExtension('codetether.codetether-rust').activate();
  assert.equal(vscode.extensions.getExtension('rust-lang.rust-analyzer'), undefined);
  const uri = document.uri;
  const source = 'mod tools { pub struct ImportableWidget; }\npub fn demo() { let _item = ImportableWid }\n';
  await replace(document, source);
  await vscode.commands.executeCommand('codetether.rust.restartNavigation');
  const completionPosition = () => document.positionAt(document.getText().indexOf('ImportableWid }') + 'ImportableWid'.length);
  const completions = await eventually(() => vscode.commands.executeCommand('vscode.executeCompletionItemProvider', uri, completionPosition(), undefined, 100),
    result => result?.items.some(item => item.additionalTextEdits?.some(edit => edit.newText.includes('use '))), 'auto-import completion with resolved edits');
  const completion = completions.items.find(item => item.additionalTextEdits?.some(edit => edit.newText.includes('ImportableWidget')));
  assert.ok(completion, 'completion includes the import for the out-of-scope type');
  const accepted = new vscode.WorkspaceEdit();
  for (const edit of completion.additionalTextEdits) accepted.replace(uri, edit.range, edit.newText);
  const range = completion.range?.replacing || completion.range;
  assert.ok(range);
  const insertion = typeof completion.insertText === 'string' ? completion.insertText : completion.insertText?.value;
  assert.equal(insertion, 'ImportableWidget');
  accepted.replace(uri, range, insertion);
  assert.ok(await vscode.workspace.applyEdit(accepted));
  assert.match(document.getText(), /use .*tools::ImportableWidget;/);
  assert.match(document.getText(), /let _item = ImportableWidget/);

  // A lightbulb import assist must work on a missing import in an unsaved buffer.
  await replace(document, source.replace('ImportableWid }', 'ImportableWidget; }'));
  const usePosition = () => document.positionAt(document.getText().lastIndexOf('ImportableWidget') + 3);
  const actions = await eventually(() => vscode.commands.executeCommand('vscode.executeCodeActionProvider', uri,
    new vscode.Range(usePosition(), usePosition()), undefined, 100),
    result => result?.some(action => /Import.*ImportableWidget/.test(action.title) && action.edit), 'resolved import assist');
  const action = actions.find(item => /Import.*ImportableWidget/.test(item.title) && item.edit);
  assert.ok(await vscode.workspace.applyEdit(action.edit));
  assert.match(document.getText(), /use .*tools::ImportableWidget;/);

  // Live issues appear before saving and clear on correction; other owners survive toggles.
  await replace(document, 'pub fn demo() { let value = ; }\n');
  const nativeIssues = () => vscode.languages.getDiagnostics(uri).filter(item => item.source === 'rust-analyzer');
  await eventually(nativeIssues, result => result.length > 0, 'live unsaved issue');
  const foreign = vscode.languages.createDiagnosticCollection('editing-test-other-owner');
  const marker = new vscode.Diagnostic(new vscode.Range(0, 0, 0, 1), 'Other owner marker');
  marker.source = 'editing-test'; foreign.set(uri, [marker]);
  try {
    await config.update('navigation.diagnostics', false, vscode.ConfigurationTarget.WorkspaceFolder);
    await eventually(nativeIssues, result => result.length === 0, 'disable clears own issues');
    assert.ok(vscode.languages.getDiagnostics(uri).some(item => item.source === 'editing-test'));
    await config.update('navigation.diagnostics', true, vscode.ConfigurationTarget.WorkspaceFolder);
    await replace(document, 'pub fn demo() { let value = ; } // changed\n');
    await eventually(nativeIssues, result => result.length > 0, 're-enable live issues');
  } finally { foreign.dispose(); }
  const valid = 'pub fn add(left: u32, right: u32) -> u32 { left + right }\npub fn demo() -> u32 { let total = add(1, 2); total }\n';
  await replace(document, valid);
  await eventually(nativeIssues, result => result.length === 0, 'correction clears issues');
  const callPosition = document.positionAt(valid.lastIndexOf('add(') + 1);
  await eventually(() => vscode.commands.executeCommand('vscode.executeHoverProvider', uri, callPosition),
    result => result?.some(item => item.contents.length > 0), 'hover');
  const signaturePosition = document.positionAt(valid.lastIndexOf('add(') + 4);
  await eventually(() => vscode.commands.executeCommand('vscode.executeSignatureHelpProvider', uri, signaturePosition),
    result => result?.signatures.some(item => item.label.includes('left')), 'signature help');
  const whole = new vscode.Range(document.positionAt(0), document.positionAt(valid.length));
  await eventually(() => vscode.commands.executeCommand('vscode.executeInlayHintProvider', uri, whole),
    result => result?.length > 0, 'inlay hints');
  await config.update('navigation.inlayHints', false, vscode.ConfigurationTarget.WorkspaceFolder);
  await eventually(() => vscode.commands.executeCommand('vscode.executeInlayHintProvider', uri, whole),
    result => result?.length === 0, 'live inlay hint toggle');
  const rename = await vscode.commands.executeCommand('vscode.executeDocumentRenameProvider', uri, callPosition, 'sum');
  assert.ok(rename && rename.get(uri).length >= 2);
  assert.ok(await vscode.workspace.applyEdit(rename));
  assert.match(document.getText(), /fn sum/); assert.match(document.getText(), /total = sum/);
  assert.ok(document.isDirty, 'all edits stayed in the editor buffer');

  await config.update('navigation.autoImport', false, vscode.ConfigurationTarget.WorkspaceFolder);
  await replace(document, source);
  await eventually(() => vscode.commands.executeCommand('vscode.executeCompletionItemProvider', uri, completionPosition(), undefined, 100),
    result => result && !result.items.some(item => item.additionalTextEdits?.some(edit => edit.newText.includes('use '))), 'disable auto-import');
  fs.writeFileSync(path.join(process.env.CODETETHER_EDITOR_EVIDENCE, 'editing.json'), JSON.stringify({
    validation: 'focused CI-like', realRustAnalyzer: true, rustAnalyzerExtensionAbsent: true,
    autoImportCompletionEditsApplied: true, importAssistApplied: true,
    unsavedIssuesPublishedAndCleared: true, diagnosticTogglePreservesOtherOwners: true,
    hover: true, signatureHelp: true, inlayHints: true, renameApplied: true,
    liveInlayHintsAndAutoImportToggles: true
  }, null, 2));
}
module.exports = { run };
