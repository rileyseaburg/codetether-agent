'use strict';
const vm = require('node:vm');
const fs = require('node:fs');
const path = require('node:path');

/** Mock only VS Code's host API, loading actual extension modules. Example: host().commands. */
function host(options = {}) {
  const commands = new Map(), tasks = [], errors = [], invoked = [], providers = [], executions = new Map();
  let onSave;
  const starts = [], finishes = [], ends = [], bars = [], output = [], updates = [];
  const config = { ...options.config };
  const event = handlers => callback => { handlers.push(callback); return { dispose() {} }; };
  const disposable = { dispose() {} };
  const folder = { uri: { fsPath: '/repo/work tree', toString: () => 'file:///repo/work%20tree' }, name: 'app' };
  const vscode = {
    commands: {
      executeCommand: async id => invoked.push(id),
      registerCommand: (id, callback) => { commands.set(id, callback); return disposable; }
    },
    workspace: {
      isTrusted: options.trusted ?? true,
      workspaceFolders: options.multi ? [folder, folder] : [folder],
      getWorkspaceFolder: () => folder,
      fs: { stat: async () => { if (options.missing) throw Error('Cargo.toml missing'); } },
      saveAll: async () => options.save ?? true,
      getConfiguration: () => ({
        get: (key, fallback) => key === 'sharedDiagnostics' ? false : config[key] ?? fallback,
        update: async (key, value, target) => { config[key] = value; updates.push({ key, value, target }); }
      }),
      onDidChangeConfiguration: () => disposable,
      onDidSaveTextDocument: callback => { onSave = callback; return disposable; },
      onDidChangeWorkspaceFolders: () => disposable,
      registerTextDocumentContentProvider: () => disposable
    },
    window: {
      createOutputChannel: () => ({ ...disposable, appendLine: line => output.push(line), show() {} }),
      createStatusBarItem: () => { const bar = { ...disposable, show() {}, hide() {} }; bars.push(bar); return bar; },
      onDidChangeActiveTextEditor: () => disposable,
      showQuickPick: async choices => choices.find(choice => choice.action === options.action),
      showWorkspaceFolderPick: async () => options.cancel ? undefined : folder,
      showWarningMessage: async () => options.confirm,
      showErrorMessage: message => errors.push(message)
    },
    languages: {
      createDiagnosticCollection: () => ({ ...disposable, clear() {}, set() {} }),
      registerCodeActionsProvider: (_, provider) => { providers.push(provider); return disposable; },
      onDidChangeDiagnostics: () => disposable,
      getDiagnostics: uri => uri ? options.diagnostics || [] : options.allDiagnostics || []
    },
    StatusBarAlignment: { Left: 1 }, DiagnosticSeverity: { Error: 0, Warning: 1 },
    ConfigurationTarget: { WorkspaceFolder: 3 },
    ThemeColor: class { constructor(id) { this.id = id; } },
    CodeActionKind: { QuickFix: 'quickfix' },
    CodeAction: class { constructor(title, kind) { this.title = title; this.kind = kind; } },
    Uri: { joinPath: (...parts) => parts },
    Task: class { constructor(...args) { this.args = args; this.definition = args[0]; } },
    ProcessExecution: class { constructor(...args) { this.args = args; } },
    TaskRevealKind: { Always: 1, Never: 3 }, TaskPanelKind: { Dedicated: 1 },
    tasks: {
      taskExecutions: [],
      onDidStartTask: event(starts),
      onDidEndTask: event(ends),
      onDidEndTaskProcess: event(finishes),
      executeTask: async task => { tasks.push(task); const execution = { task }; executions.set(task, execution); starts.forEach(handler => handler({ execution })); return execution; }
    }
  };
  const modules = new Map();
  function load(file) {
    if (modules.has(file)) return modules.get(file).exports;
    const module = { exports: {} }; modules.set(file, module);
    vm.runInNewContext(fs.readFileSync(file, 'utf8'), { module, Buffer, console,
      setTimeout, clearTimeout, setInterval: () => 0, clearInterval() {}, process,
      require: name => name === 'vscode' ? vscode : name.startsWith('.') ? load(require.resolve(path.resolve(path.dirname(file), name))) : require(name)
    }, { filename: file });
    return module.exports;
  }
  load(require.resolve('../src/extension')).activate({ subscriptions: [] });
  return { commands, tasks, errors, invoked, providers, folder, bars, output, updates,
    load: name => load(require.resolve(`../src/${name}`)),
    end: task => ends.forEach(handler => handler({ execution: executions.get(task) || { task } })),
    finish: event => finishes.forEach(handler => handler({ ...event, execution: executions.get(event.execution.task) || event.execution })),
    save: document => onSave(document) };
}
module.exports = { host };