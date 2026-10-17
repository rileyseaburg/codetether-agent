//! Generic MCP registry register operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Register a client with the registry
    pub async fn register(&self, name: &str, client: Arc<C>, capabilities: ServerCapabilities) {
        // Store client
        self.clients.write().await.insert(name.to_string(), client);

        // Store capabilities
        self.server_capabilities
            .write()
            .await
            .insert(name.to_string(), capabilities);

        tracing::info!(target: "codetether_agent::mcp::client", "Registered MCP server '{}' with registry", name);
    }
}
