//! Application identity and logging for the shared LSP handshake.

use super::{Host, LspClient};
use anyhow::Result;

impl<H: Host> LspClient<H> {
    /// Initialize the language server using this application's package version.
    /// # Returns
    /// Unit after the handshake and host initialization notification succeed.
    /// # Errors
    /// Returns serialization, transport, or server initialization errors.
    pub async fn initialize(&self) -> Result<()> {
        self.inner
            .initialize_with_identity(H::version(), H::initialized)
            .await
    }
}
