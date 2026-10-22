'use strict';
const vscode = require('vscode');
const { plan } = require('./plan');
const { environment } = require('./cache');

/** Own task lifecycle and per-folder check serialization. Example: create(context).run('check'). */
function create(context) {
  const checks = new Map(), queued = new Map(), idle = new Set();
  let saves = 0;
  const finish = event => {
    const definition = event.execution.task.definition;
    if (definition.type !== 'codetether-rust') return;
    if (definition.action === 'check') {
      const current = checks.get(definition.workspace);
      if (current?.execution && current.execution !== event.execution) return;
      checks.delete(definition.workspace);
      const pending = queued.get(definition.workspace);
      queued.delete(definition.workspace);
      if (pending) void runner.run('check', pending.folder, pending.automatic, pending.overrides).catch(error => {
        vscode.window.showErrorMessage(`CodeTether check: ${error.message}`);
      });
      for (const callback of idle) callback(definition.workspace);
    }
    if (event.exitCode && !definition.automatic) {
      vscode.commands.executeCommand('workbench.actions.view.problems');
      vscode.window.showErrorMessage(`CodeTether Rust: task exited ${event.exitCode}. See Problems and the task terminal for diagnostics.`);
    }
  };
  context.subscriptions.push(vscode.tasks.onDidEndTaskProcess(finish));
  // A failed launch can end a task without a process-exit event.
  context.subscriptions.push(vscode.tasks.onDidEndTask(event => {
    if (checks.get(event.execution.task.definition.workspace)?.execution === event.execution) finish(event);
  }));
  const runner = {
    get saving() { return saves > 0; },
    isChecking: key => checks.has(key),
    onIdle: callback => { idle.add(callback); return { dispose: () => idle.delete(callback) }; },
    async suppressSaves(operation) { saves++; try { return await operation(); } finally { saves--; } },
    async run(action, folder, automatic = false, overrides = {}) {
      if (!vscode.workspace.isTrusted) throw Error('Trust the workspace before running Rust commands.');
      if (!folder) {
        const folders = vscode.workspace.workspaceFolders || [];
        folder = folders.length === 1 ? folders[0] : await vscode.window.showWorkspaceFolderPick();
      }
      if (!folder) return;
      const key = folder.uri.toString();
      if (action === 'check' && checks.has(key)) {
        queued.set(key, { folder, automatic, overrides });
        return;
      }
      const record = {};
      if (action === 'check') checks.set(key, record);
      try {
        if (action === 'stop' && await vscode.window.showWarningMessage('Stop the shared ra-jit daemon? Other workspaces may use it.', { modal: true }, 'Stop') !== 'Stop') return;
        const sharedCache = ['cache-build', 'cache-publish'].includes(action);
        if (sharedCache) {
          if (automatic) throw Error('Shared cache actions must be explicitly requested.');
          await vscode.workspace.fs.stat(vscode.Uri.joinPath(folder.uri, 'scripts/dev/release-build-loop.sh'));
          if (await vscode.window.showWarningMessage('Run the repository release/cache pipeline? It may upload verified artifacts using host credentials. Publication can be deferred by clean-tree/receipt guards; task success alone does not prove an upload. No backend will be started.', { modal: true }, 'Continue') !== 'Continue') return;
        }
        if (['build', 'run', 'test', 'check'].includes(action) || sharedCache) {
          await vscode.workspace.fs.stat(vscode.Uri.joinPath(folder.uri, 'Cargo.toml'));
          if (!automatic && !await runner.suppressSaves(() => vscode.workspace.saveAll(false))) throw Error('Save cancelled; Rust command was not started.');
        }
        const settings = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
        const config = Object.fromEntries(['executable', 'toolchain', 'package', 'bin', 'runArgs'].map(name => [name, settings.get(name)]));
        const invocation = plan(action, folder.uri.fsPath, { ...config, ...overrides });
        const task = new vscode.Task({ type: 'codetether-rust', action, workspace: key, automatic }, folder,
          `Rust ${action}`, 'CodeTether', new vscode.ProcessExecution(invocation.executable, invocation.args,
            { cwd: invocation.cwd, env: sharedCache ? { CARGO_TERM_COLOR: 'never' } : environment(settings) }), ['$codetether-rustc']);
        task.presentationOptions = { reveal: automatic ? vscode.TaskRevealKind.Never : vscode.TaskRevealKind.Always, panel: vscode.TaskPanelKind.Dedicated };
        const execution = await vscode.tasks.executeTask(task);
        record.execution = execution;
        return execution;
      } catch (error) {
        if (action === 'check') checks.delete(key);
        throw error;
      }
    }
  };
  return runner;
}
module.exports = { create };