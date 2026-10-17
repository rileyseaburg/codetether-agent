//! capabilities operations for language server clients.

use super::*;

impl LspClient {
    /// Get the server capabilities
    pub async fn capabilities(&self) -> Option<lsp_types::ServerCapabilities> {
        self.server_capabilities.read().await.clone()
    }

    /// Check if this client handles the given file extension
    #[allow(dead_code)]
    pub fn handles_file(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        self.config.file_extensions.iter().any(|fe| fe == ext)
    }

    /// Check if this client handles a language by name
    #[allow(dead_code)]
    pub fn handles_language(&self, language: &str) -> bool {
        let extensions = match language {
            "rust" => &["rs"][..],
            "typescript" => &["ts", "tsx"],
            "javascript" => &["js", "jsx"],
            "python" => &["py"],
            "go" => &["go"],
            "c" => &["c", "h"],
            "cpp" => &["cpp", "cc", "cxx", "hpp", "h"],
            _ => &[],
        };

        extensions
            .iter()
            .any(|ext| self.config.file_extensions.iter().any(|fe| fe == *ext))
    }
}
