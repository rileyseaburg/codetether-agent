//! Initialize remote metadata and discover advertised tools.
use super::*;
impl RemoteClient {
    /// Initialize the connection with the server
    ///
    /// # Arguments
    /// client_version identifies the calling application.
    ///
    /// # Returns
    /// Initialized metadata after notification and optional tool discovery.
    ///
    /// # Errors
    /// Wire, transport, response decoding or tool discovery errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.initialize("1.0").await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn initialize(&self, client_version: &str) -> Result<InitializeResult> {
        let result = self.connection.initialize(client_version).await?;

        // Store server info
        *self.server_info.write().await = Some(result.server_info.clone());
        *self.server_capabilities.write().await = Some(result.capabilities.clone());

        // Register this client with the registry if a server name is set
        if let Some(name) = self.server_name.read().await.clone() {
            // Create a self-reference for registration
            // Note: This is called after construction, so we need to handle registration externally
            // or use a post-initialization hook
            tracing::debug!(target: "codetether_agent::mcp::client", "Client initialized with server name: {}", name);
        }

        // Send initialized notification
        self.notify("notifications/initialized", None).await?;

        tracing::info!(target: "codetether_agent::mcp::client",
            "Connected to MCP server: {} v{}",
            result.server_info.name, result.server_info.version
        );

        // Fetch available tools
        if result.capabilities.tools.is_some() {
            self.refresh_tools().await?;
        }

        Ok(result)
    }
}
