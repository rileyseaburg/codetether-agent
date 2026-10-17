//! Application identity and logging for the shared LSP handshake.

use super::LspClient;
use anyhow::Result;
use tracing::info;

impl LspClient {
    /// Initialize the language server using this application's package version.
    pub async fn initialize(&self) -> Result<()> {
        codetether_lsp_core::initialize::run(
            &self.transport,
            &self.config,
            &self.server_capabilities,
            env!("CARGO_PKG_VERSION"),
            |server_info| info!(server_info = ?server_info, "LSP server initialized"),
        )
        .await
    }
}
