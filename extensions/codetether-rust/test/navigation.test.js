'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const options = require('../src/navigation-options');
function host({ trusted = true, competing = false, fail = false } = {}) {
  const folder = name => ({ name, uri: { scheme: 'file', fsPath: `/${name}`, toString: () => `file:///${name}` } });
  const folders = [folder('one'), folder('two')], created = [], errors = [], handlers = {}, commands = {};
  const disposable = { dispose() {} };
  const values = { one: {}, two: {} };
  const event = name => callback => { handlers[name] = callback; return disposable; };
  const workspace = {
    isTrusted: trusted, workspaceFolders: folders, textDocuments: [],
    getWorkspaceFolder: uri => folders.find(folder => uri.fsPath.startsWith(folder.uri.fsPath)),
    getConfiguration: (_section, uri) => ({ get: (key, defaultValue) => values[uri.fsPath.slice(1)][key] ?? defaultValue }),
    fs: { stat: async () => ({}) }, onDidOpenTextDocument: event('open'),
    onDidChangeWorkspaceFolders: event('folders'), onDidGrantWorkspaceTrust: event('trust'),
    onDidChangeConfiguration: event('config')
  };
  class LanguageClient {
    constructor(id, name, server, client) { Object.assign(this, { id, name, server, client, stops: 0 }); created.push(this); }
    features = [];
    registerFeature(feature) { this.features.push(feature); }
    notifications = [];
    clears = 0;
    diagnostics = { clear: () => this.clears++ };
    async sendNotification(...args) { this.notifications.push(args); }
    async start() { if (fail) throw Error('missing executable'); }
    async dispose() { this.stops++; }
  }
  const vscode = { workspace,
    extensions: { getExtension: () => ({ isActive: competing }), onDidChange: event('extensions') },
    window: { createOutputChannel: () => ({ ...disposable, appendLine() {} }), showErrorMessage: text => errors.push(text) },
    Uri: { joinPath: uri => uri }, RelativePattern: class {},
    commands: { registerCommand: (name, fn) => { commands[name] = fn; return disposable; } }
  };
  const module = { exports: {} };
  vm.runInNewContext(fs.readFileSync(require.resolve('../src/navigation'), 'utf8'), {
    module, require: name => name === 'vscode' ? vscode : name === './navigation-options' ? options : { LanguageClient }
  });
  const manager = module.exports.register({ subscriptions: [] });
  return { manager, created, errors, workspace, handlers, commands, values };
}
const settle = () => new Promise(resolve => setImmediate(resolve));
test('navigation disables automatic compiler work', () => {
  assert.equal(options.settings().checkOnSave, false);
  assert.equal(options.settings().cargo.buildScripts.enable, false);
  assert.equal(options.settings().procMacro.enable, false);
  assert.deepEqual(options.configuration({ items: [{ section: 'rust-analyzer.check.enable' }, { section: 'other' }] }), [false, null]);
});
test('navigation respects trust and competing extension', async () => {
  for (const settings of [{ trusted: false }, { competing: true }]) {
    const h = host(settings); await settle(); assert.equal(h.created.length, 0); await h.manager.dispose();
  }
});
test('navigation isolates buffers and disposes clients on restart/removal', async () => {
  const h = host(); await settle(); assert.equal(h.created.length, 2);
  await h.handlers.folders(); assert.equal(h.created.length, 2);
  const first = h.created[0]; assert.equal(first.server.options.cwd, '/one');
  const own = { uri: { fsPath: '/one/src/lib.rs' } }, other = { uri: { fsPath: '/two/src/lib.rs' } };
  const m = first.client.middleware; let changes = 0;
  m.didChange({ document: own }, () => changes++); m.didChange({ document: other }, () => changes++);
  assert.equal(changes, 1);
  assert.equal(m.provideDefinition(other, {}, {}, () => 'wrong'), null);
  assert.equal(m.provideDefinition(own, {}, {}, () => 'right'), 'right');
  await h.commands['codetether.rust.restartNavigation'](); assert.equal(h.created.length, 4);
  assert.equal(first.stops, 1);
  h.workspace.workspaceFolders = []; await h.handlers.folders();
  assert.equal(h.created[2].stops, 1); await h.manager.dispose();
  assert.equal(h.created[2].stops, 1);
});
test('navigation startup failures are actionable and cleaned up', async () => {
  const h = host({ fail: true }); await settle();
  assert.match(h.errors[0], /navigation.serverPath/); assert.equal(h.created[0].stops, 1);
  await h.manager.dispose();
});
test('editing options enable imports/hints/issues without enabling compiler work', () => {
  const defaults = options.settings();
  assert.equal(defaults.completion.autoimport.enable, true);
  assert.equal(defaults.diagnostics.enable, true);
  assert.equal(defaults.inlayHints.typeHints.enable, true);
  const capabilities = { textDocument: { diagnostic: {}, publishDiagnostics: {} }, workspace: { diagnostics: {}, configuration: true } };
  options.pushDiagnosticsFeature().fillClientCapabilities(capabilities);
  assert.deepEqual(capabilities, { textDocument: { publishDiagnostics: {} }, workspace: { configuration: true } });
  const disabled = options.settings({ get: () => false });
  assert.equal(disabled.completion.autoimport.enable, false);
  assert.equal(disabled.diagnostics.enable, false);
  assert.deepEqual(Object.values(disabled.inlayHints).map(hint => hint.enable), [false, false, false]);
  assert.equal(disabled.check.enable, false);
  assert.equal(disabled.checkOnSave, false);
  assert.equal(disabled.cargo.buildScripts.enable, false);
  assert.equal(disabled.procMacro.enable, false);
});
test('editing providers and diagnostics stay in their owning workspace', async () => {
  const h = host(); await settle();
  try {
    assert.equal(h.created[0].features.length, 1);
    const m = h.created[0].client.middleware;
    const own = { uri: { fsPath: '/one/src/lib.rs' } }, foreign = { uri: { fsPath: '/two/src/lib.rs' } };
    for (const [method, args] of [
      ['provideCompletionItem', [{}, {}, {}]], ['provideCodeActions', [{}, {}, {}]],
      ['provideHover', [{}, {}]], ['provideSignatureHelp', [{}, {}, {}]],
      ['provideInlayHints', [{}, {}]], ['prepareRename', [{}, {}]],
      ['provideRenameEdits', [{}, 'renamed', {}]]
    ]) {
      assert.equal(m[method](foreign, ...args, () => 'wrong'), null, method);
      assert.equal(m[method](own, ...args, (...actual) => {
        assert.deepEqual(actual, [own, ...args]); return 'right';
      }), 'right', method);
    }
    const published = [];
    const next = (uri, diagnostics) => published.push({ uri, diagnostics });
    m.handleDiagnostics(foreign.uri, ['foreign'], next);
    m.handleDiagnostics(own.uri, ['own'], next);
    assert.deepEqual(published, [{ uri: own.uri, diagnostics: ['own'] }]);
    h.values.one['navigation.diagnostics'] = false;
    m.handleDiagnostics(own.uri, ['late'], next);
    assert.equal(published[1].diagnostics.length, 0);
  } finally { await h.manager.dispose(); }
});
test('live configuration is folder-scoped and clears only its own diagnostics without restarting', async () => {
  const h = host(); await settle();
  try {
    h.values.one['navigation.autoImport'] = false;
    h.values.one['navigation.diagnostics'] = false;
    h.values.one['navigation.inlayHints'] = false;
    await h.handlers.config({ affectsConfiguration: () => true });
    assert.equal(h.created.length, 2);
    const [one, two] = h.created;
    assert.equal(one.notifications[0][0], 'workspace/didChangeConfiguration');
    assert.equal(one.clears, 1); assert.equal(two.clears, 0);
    assert.equal(two.notifications.length, 0);
    for (const [client, expected] of [[one, false], [two, true]]) {
      const result = client.client.middleware.workspace.configuration({ items: [{ section: 'rust-analyzer.completion.autoimport.enable' }] });
      assert.deepEqual(result, [expected]);
    }
    await h.handlers.config({ affectsConfiguration: () => true });
    assert.equal(one.notifications.length, 1);
  } finally { await h.manager.dispose(); }
});