//! Refresh and read the remote tool cache.
use super::*;
impl RemoteClient {
    /// Refresh the list of available tools
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// The discovered tool list, also saved in the cache.
    ///
    /// # Errors
    /// Transport, wire or decoding errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.refresh_tools().await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn refresh_tools(&self) -> Result<Vec<McpTool>> {
        let tools = self.connection.list_tools().await?;

        *self.available_tools.write().await = tools.clone();

        tracing::info!(target: "codetether_agent::mcp::client", "Loaded {} tools from MCP server", tools.len());

        Ok(tools)
    }

    /// Get available tools
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// A clone of the cached tools.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.tools().await;
    /// Ok(())
    /// }
    /// ```
    pub async fn tools(&self) -> Vec<McpTool> {
        self.available_tools.read().await.clone()
    }
}
