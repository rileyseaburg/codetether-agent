'use strict';

/** Indexing policy, independent of rust-analyzer extension settings. */
function settings(config) {
  const get = (key, fallback) => config?.get(`navigation.${key}`, fallback) ?? fallback;
  return {
    checkOnSave: false,
    check: { enable: false },
    cargo: { buildScripts: { enable: false } },
    procMacro: { enable: false },
    diagnostics: { enable: get('diagnostics', true) },
    completion: { autoimport: { enable: get('autoImport', true) } },
    inlayHints: {
      typeHints: { enable: get('inlayHints', true) },
      parameterHints: { enable: get('inlayHints', true) },
      chainingHints: { enable: get('inlayHints', true) }
    }
  };
}

/** Answer LSP configuration requests without importing another extension's checks. */
function configuration(params, config) {
  const value = settings(config);
  return params.items.map(({ section }) => {
    if (!section || section === 'rust-analyzer') return value;
    if (!section.startsWith('rust-analyzer.')) return null;
    return section.slice('rust-analyzer.'.length).split('.')
      .reduce((item, key) => item?.[key], value) ?? null;
  });
}
/** Use one owned push collection. The library's pull path bypasses handleDiagnostics. */
function pushDiagnosticsFeature() {
  return {
    getState: () => ({ kind: 'static' }),
    fillClientCapabilities(capabilities) {
      if (capabilities.textDocument) delete capabilities.textDocument.diagnostic;
      if (capabilities.workspace) delete capabilities.workspace.diagnostics;
    },
    initialize() {},
    clear() {}
  };
}
module.exports = { settings, configuration, pushDiagnosticsFeature };