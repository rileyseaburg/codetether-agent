'use strict';
const vscode = require('vscode');
const fs = require('node:fs/promises');
const path = require('node:path');
const os = require('node:os');
const { fileURLToPath } = require('node:url');
const { decode } = require('./shared-envelope');

/** Native cache location on the remote extension host; example: directory(''). */
function directory(override) {
  const cache = process.platform === 'darwin' ? path.join(os.homedir(), 'Library/Caches')
    : process.platform === 'win32' ? process.env.LOCALAPPDATA : process.env.XDG_CACHE_HOME || path.join(os.homedir(), '.cache');
  return override || process.env.CODETETHER_DIAGNOSTICS_DIR || path.join(cache || os.homedir(), 'codetether/diagnostics');
}

/** Mirror the native LSP feed without starting a language server. Example: register(context). */
function register(context) {
  const collection = vscode.languages.createDiagnosticCollection('codetether-native-lsp');
  let disposed = false, reading = false, previous = '';
  async function refresh() {
    if (reading || disposed) return;
    reading = true;
    try {
      const result = new Map();
      if (vscode.workspace.isTrusted) for (const folder of vscode.workspace.workspaceFolders || []) {
        const config = vscode.workspace.getConfiguration('codetether.rust', folder.uri);
        if (!config.get('sharedDiagnostics', true)) continue;
        const root = await fs.realpath(folder.uri.fsPath);
        const dir = directory(config.get('diagnosticsDirectory', ''));
        const names = await fs.readdir(dir).catch(() => []);
        for (const name of names.filter(name => name.endsWith('.json')).sort().slice(0, 128)) {
          try {
            const file = path.join(dir, name), stat = await fs.lstat(file);
            if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 4 * 1024 * 1024) continue;
            const envelope = JSON.parse(await fs.readFile(file, 'utf8'));
            for (const [uri, diagnostics] of await decode(envelope, root, fs.realpath)) {
              const previous = result.get(uri) || [];
              result.set(uri, previous.concat(diagnostics));
            }
          } catch { /* Partial, stale or foreign producers cannot publish markers. */ }
        }
      }
      const signature = JSON.stringify([...result]);
      if (!disposed && signature !== previous) {
        previous = signature;
        collection.clear();
        for (const [uri, values] of result) collection.set(vscode.Uri.file(fileURLToPath(uri)), values.map(value => {
          const range = new vscode.Range(value.range.start.line, value.range.start.character, value.range.end.line, value.range.end.character);
          const diagnostic = new vscode.Diagnostic(range, value.message, value.severity - 1);
          diagnostic.source = 'CodeTether LSP'; diagnostic.code = value.code;
          return diagnostic;
        }));
      }
    } catch (error) { console.warn(`CodeTether diagnostics bridge: ${error.message}`); }
    finally { reading = false; }
  }
  const timer = setInterval(refresh, 1500);
  context.subscriptions.push(collection, { dispose() { disposed = true; clearInterval(timer); } });
  context.subscriptions.push(vscode.workspace.onDidChangeWorkspaceFolders(refresh));
  void refresh();
}
module.exports = { register, directory };