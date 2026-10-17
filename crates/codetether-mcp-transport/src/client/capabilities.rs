//! Query the initialized server capabilities.
use super::*;
impl RemoteClient {
    /// Check if the connected server has a specific capability
    ///
    /// # Arguments
    /// capability is tools, resources, prompts or logging.
    ///
    /// # Returns
    /// Whether initialized metadata advertises that capability.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.has_capability("tools").await;
    /// Ok(())
    /// }
    /// ```
    pub async fn has_capability(&self, capability: &str) -> bool {
        let caps = self.server_capabilities.read().await;
        match capability {
            "tools" => caps.as_ref().map(|c| c.tools.is_some()).unwrap_or(false),
            "resources" => caps
                .as_ref()
                .map(|c| c.resources.is_some())
                .unwrap_or(false),
            "prompts" => caps.as_ref().map(|c| c.prompts.is_some()).unwrap_or(false),
            "logging" => caps.as_ref().map(|c| c.logging.is_some()).unwrap_or(false),
            _ => false,
        }
    }

    /// Get server capabilities
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// A clone of initialized capabilities, if available.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.capabilities().await;
    /// Ok(())
    /// }
    /// ```
    pub async fn capabilities(&self) -> Option<ServerCapabilities> {
        self.server_capabilities.read().await.clone()
    }
}
