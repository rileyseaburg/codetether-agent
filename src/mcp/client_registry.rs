//! Application connection authorization for the shared MCP registry.
use super::*;
impl McpRegistry {
    /// Construct an empty registry of application MCP clients.
    pub fn new() -> Self {
        Self {
            inner: codetether_mcp_core::registry::ServerRegistry::new(),
        }
    }

    /// Connect to an MCP server and register it with the registry
    pub async fn connect(
        &self,
        name: &str,
        command: &str,
        args: &[&str],
    ) -> Result<Arc<McpClient>> {
        super::super::subprocess_policy::guard(command, args, None).await?;
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        let client = Arc::new(McpClient::with_registry(
            transport,
            Arc::new(McpRegistry::new()), // Each client gets its own registry for now
            Some(name.to_string()),
        ));

        // Start message receiver
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            client_clone.receive_loop().await;
        });

        // Initialize the connection
        let init_result = client.initialize().await?;

        // Register the client
        self.register(name, Arc::clone(&client), init_result.capabilities)
            .await;

        Ok(client)
    }
}

impl Default for McpRegistry {
    fn default() -> Self {
        Self::new()
    }
}
impl std::ops::Deref for McpRegistry {
    type Target = codetether_mcp_core::registry::ServerRegistry<McpClient>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
