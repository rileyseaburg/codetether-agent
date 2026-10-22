'use strict';
const vscode = require('vscode');
const path = require('node:path');
const { createScheduler } = require('./save-scheduler');

/** Wire optional compiler checks to saved files. Example: register(context, runner). */
function register(context, runner) {
  const settings = folder => vscode.workspace.getConfiguration('codetether.rust', folder.uri);
  const enabled = folder => vscode.workspace.isTrusted && settings(folder).get('checkOnSave', false);
  const scheduler = createScheduler({
    delay: folder => Math.max(100, Math.min(10000, settings(folder).get('checkOnSaveDelay', 750))),
    busy: key => runner.isChecking(key),
    run: async folder => { if (enabled(folder)) await runner.run('check', folder, true); },
    report: error => vscode.window.showErrorMessage(`CodeTether check on save: ${error.message}`)
  });
  context.subscriptions.push(scheduler, runner.onIdle(key => scheduler.idle(key)));
  context.subscriptions.push(vscode.workspace.onDidSaveTextDocument(document => {
    if (runner.saving || document.uri.scheme !== 'file') return;
    const folder = vscode.workspace.getWorkspaceFolder(document.uri);
    if (!folder || !enabled(folder)) return;
    const relative = path.relative(folder.uri.fsPath, document.uri.fsPath);
    if (relative.split(path.sep).some(part => ['target', 'node_modules', '.git', '.codetether'].includes(part))) return;
    if (document.languageId === 'rust' || ['Cargo.toml', 'Cargo.lock'].includes(path.basename(relative))) scheduler.schedule(folder);
  }));
  context.subscriptions.push(vscode.workspace.onDidChangeWorkspaceFolders(event => {
    for (const folder of event.removed) scheduler.cancel(folder.uri.toString());
  }));
  context.subscriptions.push(vscode.commands.registerCommand('codetether.rust.enableCheckOnSave', async selected => {
    if (!vscode.workspace.isTrusted) return vscode.window.showErrorMessage('Trust the workspace first.');
    const folders = vscode.workspace.workspaceFolders || [];
    const folder = folders.find(folder => folder.uri.toString() === selected?.uri?.toString()) ||
      (folders.length === 1 ? folders[0] : await vscode.window.showWorkspaceFolderPick());
    if (!folder) return;
    const rust = vscode.workspace.getConfiguration('rust-analyzer', folder.uri);
    const duplicate = vscode.extensions.getExtension('rust-lang.rust-analyzer') && rust.get('checkOnSave', true);
    const choice = await vscode.window.showWarningMessage(
      'Enable Cargo check on save? Checks can execute build scripts. This is not automatic source repair.',
      { modal: true }, ...(duplicate ? ['Use CodeTether checks only', 'Enable both'] : ['Enable']));
    if (!choice) return;
    if (choice === 'Use CodeTether checks only') await rust.update('checkOnSave', false, vscode.ConfigurationTarget.WorkspaceFolder);
    await settings(folder).update('checkOnSave', true, vscode.ConfigurationTarget.WorkspaceFolder);
    vscode.window.showInformationMessage('CodeTether Cargo check on save enabled for this workspace.');
  }));
}
module.exports = { register };