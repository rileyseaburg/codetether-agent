//! Application connection authorization for the shared MCP registry.
use super::*;
mod connection;
impl<H: Application> McpRegistry<H> {
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
    ) -> Result<Arc<McpClient<H>>> {
        self.inner
            .connect::<connection::Policy<H>>(name, command, args)
            .await
    }
}

impl<H: Application> Default for McpRegistry<H> {
    fn default() -> Self {
        Self::new()
    }
}
impl<H: Application> std::ops::Deref for McpRegistry<H> {
    type Target = codetether_mcp_core::registry::ServerRegistry<McpClient<H>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
