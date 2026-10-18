//! Application MCP client registry access adapter.
use super::*;
impl<H: Application> McpClient<H> {
    /// Get the registry associated with this client
    pub fn registry(&self) -> Arc<McpRegistry<H>> {
        Arc::clone(&self.registry)
    }

    /// Discover tools from the registry across all connected servers
    pub async fn discover_tools_from_registry(&self) -> Vec<(String, McpTool)> {
        self.registry.all_tools().await
    }

    /// Find a tool across all servers in the registry
    pub async fn find_tool_in_registry(&self, tool_name: &str) -> Option<(String, McpTool)> {
        self.registry.find_tool(tool_name).await
    }
}
