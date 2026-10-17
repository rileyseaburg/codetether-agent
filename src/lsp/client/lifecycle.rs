//! Construct the application identity wrapper around a shared LSP client.
use super::*;
impl LspClient {
    /// Start the configured language server.
    pub async fn new(config: LspConfig) -> Result<Self> {
        Ok(Self {
            inner: codetether_lsp_core::client::LspClient::new(config).await?,
        })
    }
    /// Start the built-in server for a language and workspace.
    pub async fn for_language(language: &str, root_uri: Option<String>) -> Result<Self> {
        Ok(Self {
            inner: codetether_lsp_core::client::LspClient::for_language(language, root_uri).await?,
        })
    }
}
