//! Server connection configuration and defaults.

use super::*;

/// LSP client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    /// Command to spawn the language server
    pub command: String,
    /// Arguments to pass to the language server
    #[serde(default)]
    pub args: Vec<String>,
    /// Root URI for the workspace
    pub root_uri: Option<String>,
    /// File extensions this server handles
    #[serde(default)]
    pub file_extensions: Vec<String>,
    /// Initialization options to pass to the server
    #[serde(default)]
    pub initialization_options: Option<Value>,
    /// Timeout for requests in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_timeout() -> u64 {
    30000
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            args: Vec::new(),
            root_uri: None,
            file_extensions: Vec::new(),
            initialization_options: None,
            timeout_ms: default_timeout(),
        }
    }
}
