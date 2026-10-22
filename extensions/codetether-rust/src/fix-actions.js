'use strict';
const vscode = require('vscode');
const { capture, fresh } = require('./fix-context');
const { propose } = require('./fix-provider');
const { create } = require('./fix-review');

/** Register explicit diagnostic repairs, never fix-on-save. Example: register(context, runner). */
function register(context, runner) {
  const review = create(context), active = new Set();
  const ours = diagnostic => ['CodeTether Rust', 'CodeTether LSP', 'rustc', 'rust-analyzer'].includes(diagnostic.source);
  context.subscriptions.push(vscode.languages.registerCodeActionsProvider({ scheme: 'file', language: 'rust' }, {
    provideCodeActions(document, range, input) {
      if (!vscode.workspace.isTrusted) return [];
      const selected = input.diagnostics.filter(ours);
      if (!selected.length) return [];
      const make = (title, diagnostics) => {
        const action = new vscode.CodeAction(title, vscode.CodeActionKind.QuickFix);
        action.diagnostics = selected;
        action.command = { command: 'codetether.rust.fixDiagnostic', title,
          arguments: [{ uri: document.uri.toString(), version: document.version, diagnostics }] };
        if (document.isDirty) action.disabled = { reason: 'Save this file before requesting a repair.' };
        return action;
      };
      const actions = [make('Fix with CodeTether…', selected.slice(0, 1))];
      const related = vscode.languages.getDiagnostics(document.uri).filter(ours);
      if (related.length > 1) actions.push(make('Fix related errors in this file with CodeTether…', related.slice(0, 50)));
      return actions;
    }
  }, { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }));
  context.subscriptions.push(vscode.commands.registerCommand('codetether.rust.fixDiagnostic', async request => {
    let key, owned = false;
    try {
      if (!request || !vscode.workspace.isTrusted) throw Error('Select a diagnostic in a trusted workspace.');
      const uri = vscode.Uri.parse(request.uri), document = await vscode.workspace.openTextDocument(uri);
      if (document.isDirty || document.version !== request.version) throw Error('The file changed. Save and request a fresh quick fix.');
      const live = vscode.languages.getDiagnostics(uri).filter(ours);
      if (!request.diagnostics?.length || !request.diagnostics.every(item => live.some(current => current.message === item.message && current.range.isEqual(item.range)))) throw Error('Diagnostic is stale. Run Check again.');
      const snapshot = await capture(uri, request.diagnostics);
      key = snapshot.folder.uri.toString();
      if (active.has(key) || runner.isChecking(key)) throw Error('Wait for the current repair/check in this workspace.');
      active.add(key); owned = true;
      const consent = await vscode.window.showWarningMessage(
        `Send ${snapshot.request.files.length} source/manifest/instruction snapshots to your configured CodeTether model for a repair proposal? No edits are applied before diff approval.`,
        { modal: true }, 'Generate proposal');
      if (consent !== 'Generate proposal') return;
      const config = vscode.workspace.getConfiguration('codetether.rust', snapshot.folder.uri);
      const proposal = await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: 'CodeTether: proposing Rust repair', cancellable: true },
        async (progress, token) => {
          progress.report({ message: 'Checking saved snapshots…' });
          await fresh(snapshot);
          if (token.isCancellationRequested) throw Error('Repair cancelled. No proposal was applied.');
          progress.report({ message: 'Waiting for the native model (up to 130 seconds)…' });
          const result = await propose(config.get('executable', 'codetether'), snapshot.request, snapshot.root, config.get('fixModel', ''), token);
          if (token.isCancellationRequested) throw Error('Repair cancelled. No proposal was applied.');
          progress.report({ message: 'Opening proposal review…' });
          return result;
        });

      await review(snapshot, proposal, runner);
    } catch (error) { vscode.window.showErrorMessage(`CodeTether repair: ${error.message}`); }
    finally { if (owned) active.delete(key); }
  }));
}
module.exports = { register };