//! Identity-aware initialization for a single language-server client.
use super::*;
impl LspClient {
    /// Initialize using caller-supplied client identity and logging.
    ///
    /// # Arguments
    /// * `client_version` — Host application version advertised to the server.
    /// * `on_initialized` — Called after capabilities have been stored.
    /// # Returns
    /// Unit after the initialized notification and transport state update.
    /// # Examples
    /// See [LspClient] for a complete initialization example.
    /// # Errors
    /// Returns serialization, transport or server initialization errors.
    pub async fn initialize_with_identity(
        &self,
        client_version: &str,
        on_initialized: impl FnOnce(&Option<ServerInfo>),
    ) -> Result<()> {
        crate::initialize::run(
            &self.transport,
            &self.config,
            &self.server_capabilities,
            client_version,
            on_initialized,
        )
        .await
    }
}
