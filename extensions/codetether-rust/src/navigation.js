'use strict';
const vscode = require('vscode');
const { settings, configuration, pushDiagnosticsFeature } = require('./navigation-options');

/** Manage one persistent, buffer-aware language client per trusted Cargo folder. */
function register(context) {
  const clients = new Map();
  const output = vscode.window.createOutputChannel('CodeTether Rust Navigation');
  let disposed = false, queue = Promise.resolve();

  /** Serialize start/stop, including configuration changes during initialization. */
  function schedule() {
    queue = queue.then(reconcile).catch(error => {
      output.appendLine(`Navigation: ${error.message}`);
      void vscode.window.showErrorMessage(`CodeTether Rust navigation: ${error.message}`);
    });
    return queue;
  }

  async function reconcile() {
    const desired = new Map();
    const competing = vscode.extensions.getExtension('rust-lang.rust-analyzer')?.isActive;
    if (!disposed && vscode.workspace.isTrusted && !competing) {
      for (const folder of vscode.workspace.workspaceFolders || []) {
        if (folder.uri.scheme !== 'file') continue;
        const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
        if (!config.get('navigation.enabled', true)) continue;
        const opened = vscode.workspace.textDocuments.some(document => document.languageId === 'rust'
          && vscode.workspace.getWorkspaceFolder(document.uri)?.uri.toString() === folder.uri.toString());
        if (!opened) {
          try { await vscode.workspace.fs.stat(vscode.Uri.joinPath(folder.uri, 'Cargo.toml')); }
          catch { continue; }
        }
        desired.set(folder.uri.toString(), { folder, executable: config.get('navigation.serverPath', 'rust-analyzer'), options: settings(config) });
      }
    }
    for (const [key, entry] of clients) {
      if (desired.get(key)?.executable === entry.executable) continue;
      await entry.client.dispose();
      clients.delete(key);
    }
    for (const [key, { folder, executable, options }] of desired) {
      if (disposed) continue;
      const current = clients.get(key);
      if (current) {
        if (JSON.stringify(current.options) !== JSON.stringify(options)) {
          await current.client.sendNotification('workspace/didChangeConfiguration', { settings: null });
          if (!options.diagnostics.enable) current.client.diagnostics?.clear();
          current.options = options;
        }
        continue;
      }
      const { LanguageClient } = require('vscode-languageclient/node');
      const owns = document => vscode.workspace.getWorkspaceFolder(document.uri)?.uri.toString() === key;
      const client = new LanguageClient('codetether-rust-navigation', `CodeTether Rust (${folder.name})`,
        { command: executable, args: [], options: { cwd: folder.uri.fsPath, shell: false } }, {
          workspaceFolder: folder,
          documentSelector: [{ scheme: 'file', language: 'rust', pattern: new vscode.RelativePattern(folder, '**/*') }],
          outputChannel: output,
          diagnosticCollectionName: 'codetether-rust-navigation',
          initializationOptions: options,
          middleware: {
            workspace: { configuration: params => configuration(params, vscode.workspace.getConfiguration('codetether.rust', folder.uri)) },
            didOpen: (doc, next) => owns(doc) ? next(doc) : undefined,
            didChange: (event, next) => owns(event.document) ? next(event) : undefined,
            didClose: (doc, next) => owns(doc) ? next(doc) : undefined,
            didSave: (doc, next) => owns(doc) ? next(doc) : undefined,
            provideTypeDefinition: (doc, pos, token, next) => owns(doc) ? next(doc, pos, token) : null,
            // Never clear or replace the compiler/TUI diagnostic collections.
            handleDiagnostics: (uri, diagnostics, next) => {
              if (owns({ uri })) next(uri, vscode.workspace.getConfiguration('codetether.rust', folder.uri)
                .get('navigation.diagnostics', true) ? diagnostics : []);
            },
            provideCompletionItem: (doc, pos, ctx, token, next) => owns(doc) ? next(doc, pos, ctx, token) : null,
            provideCodeActions: (doc, range, ctx, token, next) => owns(doc) ? next(doc, range, ctx, token) : null,
            provideHover: (doc, pos, token, next) => owns(doc) ? next(doc, pos, token) : null,
            provideSignatureHelp: (doc, pos, ctx, token, next) => owns(doc) ? next(doc, pos, ctx, token) : null,
            provideInlayHints: (doc, range, token, next) => owns(doc) ? next(doc, range, token) : null,
            prepareRename: (doc, pos, token, next) => owns(doc) ? next(doc, pos, token) : null,
            provideRenameEdits: (doc, pos, name, token, next) => owns(doc) ? next(doc, pos, name, token) : null,
            provideDefinition: (doc, pos, token, next) => owns(doc) ? next(doc, pos, token) : null,
            provideImplementation: (doc, pos, token, next) => owns(doc) ? next(doc, pos, token) : null,
            provideReferences: (doc, pos, ctx, token, next) => owns(doc) ? next(doc, pos, ctx, token) : null,
            provideDocumentSymbols: (doc, token, next) => owns(doc) ? next(doc, token) : null
          }
        });
      client.registerFeature(pushDiagnosticsFeature());
      clients.set(key, { client, executable, options });
      try {
        await client.start();
        output.appendLine(`Navigation ready: ${folder.name}; Cargo checks/build scripts/proc macros disabled.`);
      } catch (error) {
        await client.dispose();
        clients.delete(key);
        throw new Error(`${folder.name}: cannot start ${executable}. Install rust-analyzer on the extension host or set codetether.rust.navigation.serverPath. ${error.message}`);
      }
    }
  }

  context.subscriptions.push(output,
    vscode.workspace.onDidOpenTextDocument(document => { if (document.languageId === 'rust') void schedule(); }),
    vscode.workspace.onDidChangeWorkspaceFolders(schedule),
    vscode.workspace.onDidGrantWorkspaceTrust(schedule),
    vscode.extensions.onDidChange(schedule),
    vscode.workspace.onDidChangeConfiguration(event => {
      if (event.affectsConfiguration('codetether.rust.navigation')) return schedule();
    }),
    vscode.commands.registerCommand('codetether.rust.restartNavigation', async () => {
      queue = queue.then(async () => {
        for (const { client } of clients.values()) await client.dispose();
        clients.clear();
      });
      return schedule();
    }));
  void schedule();
  return { async dispose() { disposed = true; await schedule(); } };
}
module.exports = { register };