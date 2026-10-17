//! Apply application configuration to server connections.

use super::*;

/// Convert a config `LspServerEntry` into an `LspConfig`.
impl LspConfig {
    pub fn from_server_entry(
        entry: &codetether_config::LspServerEntry,
        root_uri: Option<String>,
    ) -> Self {
        Self {
            command: entry.command.clone(),
            args: entry.args.clone(),
            root_uri,
            file_extensions: entry.file_extensions.clone(),
            initialization_options: entry.initialization_options.clone(),
            timeout_ms: entry.timeout_ms,
        }
    }

    pub fn from_linter_entry(
        name: &str,
        entry: &codetether_config::LspLinterEntry,
        root_uri: Option<String>,
    ) -> Option<Self> {
        // Start from the built-in config if the linter name is known,
        // then overlay any user overrides.
        let mut base = if let Some(builtin) = get_linter_server_config(name) {
            builtin
        } else {
            // Fully custom linter — command is required.
            let command = entry.command.as_ref()?;
            LspConfig {
                command: command.clone(),
                ..Default::default()
            }
        };

        // User overrides
        if let Some(cmd) = &entry.command {
            base.command = cmd.clone();
        }
        if !entry.args.is_empty() {
            base.args = entry.args.clone();
        }
        if !entry.file_extensions.is_empty() {
            base.file_extensions = entry.file_extensions.clone();
        }
        if entry.initialization_options.is_some() {
            base.initialization_options = entry.initialization_options.clone();
        }
        base.root_uri = root_uri;
        Some(base)
    }
}
