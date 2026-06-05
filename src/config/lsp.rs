use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LSP language-server and linter settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LspSettings {
    #[serde(default)]
    pub servers: HashMap<String, LspServerEntry>,
    #[serde(default)]
    pub linters: HashMap<String, LspLinterEntry>,
    #[serde(default)]
    pub disable_builtin_linters: bool,
}

/// User-defined language server entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspServerEntry {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub file_extensions: Vec<String>,
    #[serde(default)]
    pub initialization_options: Option<serde_json::Value>,
    #[serde(default = "default_lsp_timeout")]
    pub timeout_ms: u64,
}

/// Linter server entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspLinterEntry {
    pub command: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub file_extensions: Vec<String>,
    #[serde(default)]
    pub initialization_options: Option<serde_json::Value>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_lsp_timeout() -> u64 {
    30_000
}

fn default_true() -> bool {
    true
}
