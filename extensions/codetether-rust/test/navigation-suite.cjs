'use strict';
const vscode = require('vscode');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
async function eventually(operation, predicate, label) {
  const end = Date.now() + 90000;
  while (Date.now() < end) {
    const result = await operation();
    if (predicate(result || [])) return result;
    await new Promise(resolve => setTimeout(resolve, 250));
  }
  throw Error(`Navigation timeout: ${label}`);
}
async function replace(document, text) {
  const edit = new vscode.WorkspaceEdit();
  edit.replace(document.uri, new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length)), text);
  assert.ok(await vscode.workspace.applyEdit(edit));
}
function uriOf(location) { return (location.targetUri || location.uri).toString(); }
/** Real rust-analyzer and actual VS Code providers, without mocked replies. */
async function run() {
  const folder = vscode.workspace.workspaceFolders[0];
  const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
  await config.update('checkOnSave', false, vscode.ConfigurationTarget.Global);
  assert.equal(vscode.extensions.getExtension('rust-lang.rust-analyzer'), undefined);
  const uri = vscode.Uri.joinPath(folder.uri, 'src/lib.rs');
  const document = await vscode.workspace.openTextDocument(uri);
  await vscode.window.showTextDocument(document);
  const text = 'pub trait Example { fn value(&self) -> u32; }\npub struct Item;\nimpl Example for Item { fn value(&self) -> u32 { 1 } }\npub fn destination() -> u32 { 7 }\npub fn caller() -> u32 { destination() }\n';
  await replace(document, text); await document.save();
  await vscode.commands.executeCommand('codetether.rust.restartNavigation');
  const position = needle => document.positionAt(document.getText().lastIndexOf(needle) + 2);
  const defs = await eventually(() => vscode.commands.executeCommand('vscode.executeDefinitionProvider', uri, position('destination()')),
    result => result.some(item => uriOf(item) === uri.toString()), 'definition');
  assert.equal((defs[0].targetSelectionRange || defs[0].range).start.line, 3);
  await eventually(() => vscode.commands.executeCommand('vscode.executeImplementationProvider', uri, new vscode.Position(0, 12)),
    result => result.some(item => uriOf(item) === uri.toString()), 'implementation');
  await eventually(() => vscode.commands.executeCommand('vscode.executeReferenceProvider', uri, position('destination()')),
    result => result.length >= 2, 'references');
  await eventually(() => vscode.commands.executeCommand('vscode.executeDocumentSymbolProvider', uri),
    result => result.some(item => item.name === 'destination'), 'document symbols');
  await eventually(() => vscode.commands.executeCommand('vscode.executeWorkspaceSymbolProvider', 'destination'),
    result => result.some(item => item.name === 'destination'), 'workspace symbols');
  await replace(document, '// unsaved line\n' + text.replaceAll('destination', 'unsaved_target'));
  assert.ok(document.isDirty);
  await eventually(() => vscode.commands.executeCommand('vscode.executeDefinitionProvider', uri, position('unsaved_target()')),
    result => result.some(item => (item.targetSelectionRange || item.range).start.line === 4), 'unsaved definition');
  await eventually(() => vscode.commands.executeCommand('vscode.executeDocumentSymbolProvider', uri),
    result => result.some(item => item.name === 'unsaved_target'), 'unsaved symbols');
  assert.ok(!fs.readFileSync(uri.fsPath, 'utf8').includes('unsaved_target'));
  const evidence = process.env.CODETETHER_EDITOR_EVIDENCE;
  const second = path.join(evidence, 'second-root'); fs.mkdirSync(path.join(second, 'src'), { recursive: true });
  fs.writeFileSync(path.join(second, 'Cargo.toml'), '[workspace]\n[package]\nname="second-root"\nversion="0.1.0"\nedition="2024"\n');
  fs.writeFileSync(path.join(second, 'src/lib.rs'), 'pub fn second_root_symbol() {}\npub fn use_it() { second_root_symbol(); }\n');
  assert.ok(vscode.workspace.updateWorkspaceFolders(1, 0, { uri: vscode.Uri.file(second) }));
  const secondUri = vscode.Uri.file(path.join(second, 'src/lib.rs'));
  await vscode.workspace.openTextDocument(secondUri);
  await eventually(() => vscode.commands.executeCommand('vscode.executeDefinitionProvider', secondUri, new vscode.Position(1, 22)),
    result => result.length === 1 && uriOf(result[0]) === secondUri.toString(), 'second-root definition');
  await eventually(() => vscode.commands.executeCommand('vscode.executeWorkspaceSymbolProvider', 'second_root_symbol'),
    result => result.length === 1 && result[0].location.uri.toString() === secondUri.toString(), 'second-root symbol');
  fs.writeFileSync(path.join(evidence, 'navigation.json'), JSON.stringify({
    validation: 'focused CI-like', realRustAnalyzer: true, rustAnalyzerExtensionAbsent: true,
    definition: true, implementation: true, references: true, documentSymbols: true,
    workspaceSymbols: true, unsavedDefinitionAndSymbols: true, multipleRoots: true
  }, null, 2));
}
module.exports = { run };