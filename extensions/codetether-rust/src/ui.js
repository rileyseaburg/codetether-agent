'use strict';
const vscode = require('vscode');
const state = require('./ui-state');
const cache = require('./cache');

/** Independent workspace status/actions UI; does not start a language server. */
function register(context, runner) {
  const model = state.create();
  const output = vscode.window.createOutputChannel('CodeTether Rust');
  const item = vscode.window.createStatusBarItem('codetether.rust', vscode.StatusBarAlignment.Left, 10);
  item.name = 'CodeTether Rust'; item.command = 'codetether.rust.menu';
  const current = () => {
    const uri = vscode.window.activeTextEditor?.document.uri;
    return (uri && vscode.workspace.getWorkspaceFolder(uri)) ||
      (vscode.workspace.workspaceFolders?.length === 1 ? vscode.workspace.workspaceFolders[0] : undefined);
  };
  function counts(folder) {
    let errors = 0, warnings = 0;
    for (const [uri, values] of vscode.languages.getDiagnostics()) {
      if (vscode.workspace.getWorkspaceFolder(uri)?.uri.toString() !== folder.uri.toString()) continue;
      for (const value of values) if (['CodeTether Rust', 'CodeTether LSP'].includes(value.source)) {
        if (value.severity === vscode.DiagnosticSeverity.Error) errors++;
        if (value.severity === vscode.DiagnosticSeverity.Warning) warnings++;
      }
    }
    return { errors, warnings };
  }
  function render() {
    if (!vscode.workspace.workspaceFolders?.length) { item.hide(); return; }
    item.show(); item.backgroundColor = undefined;
    const folder = current();
    if (!folder) { item.text = '$(tools) CodeTether Rust'; item.tooltip = 'Select a workspace to view Rust actions'; return; }
    const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
    const status = model.get(folder.uri.toString()), diagnostics = counts(folder);
    const last = status.last;
    const label = !vscode.workspace.isTrusted ? 'Restricted' : status.running.length ? status.running.join(', ')
      : !last ? 'Idle' : last.exitCode === 0 ? (last.action.startsWith('cache-') ? `${last.action} finished; see publication log` : `${last.action} succeeded`) : last.exitCode === undefined ? `${last.action} ended` : `${last.action} failed`;
    const icon = status.running.length ? 'sync~spin' : last?.exitCode ? 'error' : 'tools';
    item.text = `$(${icon}) CodeTether Rust: ${label}  $(error) ${diagnostics.errors} $(warning) ${diagnostics.warnings}`;
    if (!status.running.length && last?.exitCode) item.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
    item.tooltip = `${folder.name}\n${label}\nCheck on save: ${config.get('checkOnSave', false) ? 'on' : 'off'}\n` +
      `Native diagnostics: ${config.get('sharedDiagnostics', true) ? 'enabled (requires a native publisher)' : 'off'}\n` +
      `Cache: ${config.get('useSccache', false) ? 'sccache requested; inspect Cache Statistics for backend' : 'inherited Cargo configuration'}\nClick for actions. Completion/navigation is not provided by this extension.`;
    item.accessibilityInformation = { label: `CodeTether Rust, ${folder.name}, ${label}, ${diagnostics.errors} errors, ${diagnostics.warnings} warnings` };
  }
  async function menu() {
    const folder = current() || await vscode.window.showWorkspaceFolderPick();
    if (!folder) return;
    const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
    const choices = [
      ['Check (rustc)', 'check'], ['Build (ra-jit)', 'build'], ['Run (ra-jit)', 'run'], ['Test (ra-jit)', 'test'],
      ['Build & Publish Shared Cache…', 'cache-build'], ['Publish Verified Cache…', 'cache-publish'],
      ['Open Problems', 'problems'], ['Fix diagnostic at cursor…', 'fix'],
      [config.get('checkOnSave', false) ? 'Disable check on save' : 'Enable check on save…', 'save'],
      [config.get('useSccache', false) ? 'Use inherited Cargo cache settings' : 'Use sccache (existing MinIO/S3 configuration)…', 'cache'],
      ['Cache Statistics', 'stats'], ['Daemon Status', 'status'], ['Diagnose Prerequisites', 'doctor'],
      ['Show CodeTether Rust Output', 'output'], ['Settings', 'settings'], ['Stop ra-jit Daemon…', 'stop']
    ].map(([label, action]) => ({ label, action }));
    const selected = await vscode.window.showQuickPick(choices, { title: `CodeTether Rust — ${folder.name}`, placeHolder: 'Select an action; builds/checks may execute workspace code' });
    if (!selected) return;
    if (selected.action === 'problems') return vscode.commands.executeCommand('workbench.actions.view.problems');
    if (selected.action === 'output') return output.show(true);
    if (selected.action === 'settings') return vscode.commands.executeCommand('workbench.action.openSettings', '@ext:codetether.codetether-rust');
    if (!vscode.workspace.isTrusted) throw Error('Trust the workspace before running Rust commands.');
    if (selected.action === 'fix') return vscode.commands.executeCommand('editor.action.quickFix');
    if (selected.action === 'save') {
      if (config.get('checkOnSave', false)) return config.update('checkOnSave', false, vscode.ConfigurationTarget.WorkspaceFolder);
      return vscode.commands.executeCommand('codetether.rust.enableCheckOnSave', folder);
    }
    if (selected.action === 'cache') {
      const enabled = config.get('useSccache', false);
      if (!enabled && await vscode.window.showWarningMessage('Use sccache for Cargo subprocesses in this workspace? Uses existing host-side MinIO/S3 credentials and disables incremental compilation for these tasks. No cache daemon is restarted.', { modal: true }, 'Enable sccache') !== 'Enable sccache') return;
      return config.update('useSccache', !enabled, vscode.ConfigurationTarget.WorkspaceFolder);
    }
    if (selected.action === 'stats') {
      const value = await cache.statistics(config.get('sccacheExecutable', 'sccache'), folder.uri.fsPath);
      output.appendLine(`Cache backend: ${value.backend}; hits: ${value.hits}; misses: ${value.misses}; requests: ${value.requests}; cache I/O errors: ${value.errors}`);
      output.appendLine('Daemon-wide totals, not proof of hits by this workspace. ra-jit in-memory reuse is separate.');
      return output.show(true);
    }
    return runner.run(selected.action, folder);
  }
  context.subscriptions.push(item, output,
    vscode.commands.registerCommand('codetether.rust.menu', () => menu().catch(error => vscode.window.showErrorMessage(`CodeTether Rust: ${error.message}`))),
    vscode.window.onDidChangeActiveTextEditor(render), vscode.workspace.onDidChangeWorkspaceFolders(render),
    vscode.workspace.onDidChangeConfiguration(render), vscode.languages.onDidChangeDiagnostics(render),
    vscode.tasks.onDidStartTask(event => { model.start(event.execution); render(); }),
    vscode.tasks.onDidEndTaskProcess(event => {
      model.end(event.execution, event.exitCode);
      if (event.execution.task.definition.type === 'codetether-rust') output.appendLine(`${event.execution.task.name}: exit ${event.exitCode ?? 'unknown'}`);
      render();
    }),
    vscode.tasks.onDidEndTask(event => { model.end(event.execution, undefined); render(); }));
  for (const execution of vscode.tasks.taskExecutions) model.start(execution);
  render();
}
module.exports = { register };