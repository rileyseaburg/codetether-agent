//! Application identity and logging for the shared LSP handshake.

use super::LspClient;
use anyhow::Result;
use tracing::info;

impl LspClient {
    /// Initialize the language server using this application's package version.
    pub async fn initialize(&self) -> Result<()> {
        self.inner
            .initialize_with_identity(
                env!("CARGO_PKG_VERSION"),
                |server_info| info!(server_info = ?server_info, "LSP server initialized"),
            )
            .await
    }
}
