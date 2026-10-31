'use strict';
const vscode = require('vscode');
const { actions } = require('./plan');
const tasks = require('./tasks');
const onSave = require('./on-save');
const shared = require('./shared-diagnostics');
const fixes = require('./fix-actions');
const ui = require('./ui');
const navigation = require('./navigation');
let navigationHost;

/** Register first-party checks, diagnostics and reviewed fixes. Example: activate(context). */
function activate(context) {
  const runner = tasks.create(context);
  onSave.register(context, runner);
  shared.register(context);
  fixes.register(context, runner);
  ui.register(context, runner);
  navigationHost = navigation.register(context);
  for (const action of actions) {
    context.subscriptions.push(vscode.commands.registerCommand(`codetether.rust.${action}`, async () => {
      try { await runner.run(action); }
      catch (error) { vscode.window.showErrorMessage(`CodeTether Rust: ${error.message}`); }
    }));
  }
}
/** Await language-server shutdown when the extension host unloads. */
async function deactivate() { await navigationHost?.dispose(); }
module.exports = { activate, deactivate };