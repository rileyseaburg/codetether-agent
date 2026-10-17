//! lifecycle operations for language server clients.

use super::*;

impl LspClient {
    /// Create a new LSP client with the given configuration
    pub async fn new(config: LspConfig) -> Result<Self> {
        super::super::types::ensure_server_installed(&config).await?;

        let transport =
            LspTransport::spawn(&config.command, &config.args, config.timeout_ms).await?;

        Ok(Self {
            last_used: std::sync::Mutex::new(std::time::Instant::now()),
            transport,
            config,
            server_capabilities: RwLock::new(None),
            open_documents: RwLock::new(HashMap::new()),
        })
    }

    /// Create an LSP client for a specific language
    pub async fn for_language(language: &str, root_uri: Option<String>) -> Result<Self> {
        let mut config = get_language_server_config(language)
            .ok_or_else(|| anyhow::anyhow!("Unknown language: {}", language))?;
        config.root_uri = root_uri;
        Self::new(config).await
    }

    /// Shutdown the language server
    #[allow(dead_code)]
    pub async fn shutdown(&self) -> Result<()> {
        let response = self.transport.request("shutdown", None).await?;

        if let Some(error) = response.error {
            warn!(target: "codetether_agent::lsp::client::lifecycle", "LSP shutdown error: {}", error.message);
        }

        self.transport.notify("exit", None).await?;
        info!(target: "codetether_agent::lsp::client::lifecycle", "LSP server shutdown complete");

        Ok(())
    }
}
