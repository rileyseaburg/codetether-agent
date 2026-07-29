//! Verifies the `[lsp.linters.tetherscript]` override documented in AGENTS.md.

use super::tetherscript::TETHERSCRIPT_LINTER;
use super::types::LspConfig;
use crate::config::LspLinterEntry;

#[test]
fn documented_config_override_replaces_the_builtin_command() {
    let entry = LspLinterEntry {
        command: Some("/opt/custom/tetherscript".to_string()),
        args: vec!["lsp".to_string(), "--verbose".to_string()],
        file_extensions: Vec::new(),
        initialization_options: None,
        enabled: true,
    };
    let config = LspConfig::from_linter_entry(TETHERSCRIPT_LINTER, &entry, None)
        .expect("override should resolve");
    assert_eq!(config.command, "/opt/custom/tetherscript");
    assert_eq!(config.args, vec!["lsp", "--verbose"]);
    // Extensions fall back to the builtin set when the user omits them.
    assert!(config.file_extensions.contains(&"tether".to_string()));
}
