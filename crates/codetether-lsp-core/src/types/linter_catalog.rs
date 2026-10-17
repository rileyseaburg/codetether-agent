//! LSP linter catalog definitions and helpers.

use super::*;

/// Built-in linter server configurations.
/// Returns an `LspConfig` for well-known linter language servers.
pub fn get_linter_server_config(name: &str) -> Option<LspConfig> {
    match name {
        "eslint" => Some(LspConfig {
            command: "vscode-eslint-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            file_extensions: vec![
                "js".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "mjs".to_string(),
                "cjs".to_string(),
            ],
            ..Default::default()
        }),
        "biome" => Some(LspConfig {
            command: "biome".to_string(),
            args: vec!["lsp-proxy".to_string()],
            file_extensions: vec![
                "js".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "json".to_string(),
                "css".to_string(),
            ],
            ..Default::default()
        }),
        "ruff" => Some(LspConfig {
            command: "ruff".to_string(),
            args: vec!["server".to_string()],
            file_extensions: vec!["py".to_string(), "pyi".to_string()],
            ..Default::default()
        }),
        "stylelint" => Some(LspConfig {
            command: "stylelint-lsp".to_string(),
            args: vec!["--stdio".to_string()],
            file_extensions: vec!["css".to_string(), "scss".to_string(), "less".to_string()],
            ..Default::default()
        }),
        _ => super::tetherscript::linter_config(name),
    }
}
