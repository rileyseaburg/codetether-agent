//! Application connection authorization for the shared MCP registry.
use super::*;
mod connection;
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
        self.inner
            .connect::<connection::Native>(name, command, args)
            .await
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
