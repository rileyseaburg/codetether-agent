//! Built-in language server configurations.

use super::*;

/// Known language server configurations
pub fn get_language_server_config(language: &str) -> Option<LspConfig> {
    match language {
        "rust" => Some(LspConfig {
            command: rust_analyzer_command(),
            args: rust_analyzer_args(),
            file_extensions: vec!["rs".to_string()],
            timeout_ms: rust_timeout(),
            ..Default::default()
        }),
        "typescript" | "javascript" => Some(LspConfig {
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            file_extensions: vec![
                "ts".to_string(),
                "tsx".to_string(),
                "js".to_string(),
                "jsx".to_string(),
            ],
            ..Default::default()
        }),
        "python" => Some(LspConfig {
            command: "pylsp".to_string(),
            args: vec![],
            file_extensions: vec!["py".to_string()],
            ..Default::default()
        }),
        "go" => Some(LspConfig {
            command: "gopls".to_string(),
            args: vec!["serve".to_string()],
            file_extensions: vec!["go".to_string()],
            ..Default::default()
        }),
        "c" | "cpp" | "c++" => Some(LspConfig {
            command: "clangd".to_string(),
            args: vec![],
            file_extensions: vec![
                "c".to_string(),
                "cpp".to_string(),
                "cc".to_string(),
                "cxx".to_string(),
                "h".to_string(),
                "hpp".to_string(),
            ],
            ..Default::default()
        }),
        _ => super::tetherscript::language_server_config(language),
    }
}
