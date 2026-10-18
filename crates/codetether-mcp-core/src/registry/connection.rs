//! Ordered authorization, initialization, and registration of MCP connections.
use super::{RegistryClient, ServerRegistry};
use crate::types::InitializeResult;
use anyhow::Result;
use std::{future::Future, sync::Arc};

/// Native operations required to establish a registry connection.
///
/// # Implementors
/// Applications retain process policy, transport construction, and receiver
/// execution. The registry owns the order in which these operations run.
pub trait ConnectionHost {
    /// Authorized client type stored in the registry.
    type Client: RegistryClient;
    /// Authorize process startup before constructing any transport.
    fn authorize(command: &str, args: &[&str]) -> impl Future<Output = Result<()>>;
    /// Construct the transport and client using its independent child registry.
    fn create(
        name: &str,
        command: &str,
        args: &[&str],
    ) -> impl Future<Output = Result<Arc<Self::Client>>>;
    /// Start receiving messages before sending the initialization request.
    fn start_receiver(client: Arc<Self::Client>);
    /// Perform the protocol initialization exchange.
    fn initialize(client: &Self::Client) -> impl Future<Output = Result<InitializeResult>>;
}

impl<C: RegistryClient> ServerRegistry<C> {
    /// Authorize, initialize, and register one MCP server connection.
    ///
    /// # Arguments
    /// * `name` — Registry name for the server.
    /// * `command` — Executable used by the native transport.
    /// * `args` — Arguments passed to that executable.
    /// # Returns
    /// The initialized client after storing its advertised capabilities.
    /// # Errors
    /// Propagates authorization, transport creation, and initialization failures.
    /// # Examples
    /// ```no_run
    /// # async fn example<H: codetether_mcp_core::registry::ConnectionHost>(registry: &codetether_mcp_core::registry::ServerRegistry<H::Client>) -> anyhow::Result<()> {
    /// let _client = registry.connect::<H>("files", "server", &[]).await?;
    /// # Ok(()) }
    /// ```
    pub async fn connect<H: ConnectionHost<Client = C>>(
        &self,
        name: &str,
        command: &str,
        args: &[&str],
    ) -> Result<Arc<C>> {
        H::authorize(command, args).await?;
        let client = H::create(name, command, args).await?;
        H::start_receiver(Arc::clone(&client));
        let init_result = H::initialize(&client).await?;
        self.register(name, Arc::clone(&client), init_result.capabilities)
            .await;
        Ok(client)
    }
}
