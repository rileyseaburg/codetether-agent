//! Application MCP client metadata adapter.
use super::*;
impl<H: Application> McpClient<H> {
    /// Get the server name if set
    pub async fn server_name(&self) -> Option<String> {
        self.inner.server_name().await
    }

    /// Set the server name for registry tracking
    pub async fn set_server_name(&self, name: String) {
        self.inner.set_server_name(name).await
    }

    /// Check if the connected server has a specific capability
    pub async fn has_capability(&self, capability: &str) -> bool {
        self.inner.has_capability(capability).await
    }

    /// Get server capabilities
    pub async fn capabilities(&self) -> Option<ServerCapabilities> {
        self.inner.capabilities().await
    }

    /// Get available tools
    pub async fn tools(&self) -> Vec<McpTool> {
        self.inner.tools().await
    }
}
