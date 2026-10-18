//! Construct the application identity wrapper around a shared LSP client.
use super::*;
impl<H: Host> LspClient<H> {
    /// Start the configured language server.
    /// # Arguments
    /// * `config` — Server command, arguments, and workspace configuration.
    /// # Returns
    /// A connected client, ready for [`Self::initialize`].
    /// # Errors
    /// Returns server discovery or process startup errors.
    pub async fn new(config: LspConfig) -> Result<Self> {
        Ok(Self {
            host: PhantomData,
            inner: codetether_lsp_core::client::LspClient::new(config).await?,
        })
    }
    /// Start the built-in server for a language and workspace.
    /// # Arguments
    /// * `language` — Language used to select the configured server.
    /// * `root_uri` — Optional workspace URI passed to the server.
    /// # Returns
    /// A connected client, ready for [`Self::initialize`].
    /// # Errors
    /// Returns unknown-language, server discovery, or process startup errors.
    pub async fn for_language(language: &str, root_uri: Option<String>) -> Result<Self> {
        Ok(Self {
            host: PhantomData,
            inner: codetether_lsp_core::client::LspClient::for_language(language, root_uri).await?,
        })
    }
}
