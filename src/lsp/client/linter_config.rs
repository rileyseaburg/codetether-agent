//! linter config operations for language server clients.

use super::*;

impl LspManager {
    pub(super) fn resolve_linter_config(&self, name: &str) -> Option<LspConfig> {
        // Resolve config
        let config = if let Some(settings) = &self.lsp_settings {
            if let Some(entry) = settings.linters.get(name) {
                if !entry.enabled {
                    return None;
                }
                LspConfig::from_linter_entry(name, entry, self.root_uri.clone())
            } else if !settings.disable_builtin_linters {
                // Not explicitly configured — try built-in
                if let Some(mut cfg) = get_linter_server_config(name) {
                    cfg.root_uri = self.root_uri.clone();
                    Some(cfg)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // No config provided — use built-in defaults
            if let Some(mut cfg) = get_linter_server_config(name) {
                cfg.root_uri = self.root_uri.clone();
                Some(cfg)
            } else {
                None
            }
        };

        config
    }
}
