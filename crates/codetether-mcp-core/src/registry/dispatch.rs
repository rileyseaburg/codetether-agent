//! Generic MCP registry dispatch operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Call a tool on a specific server
    pub async fn call_tool(
        &self,
        server: &str,
        tool: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let client = self
            .get(server)
            .await
            .ok_or_else(|| anyhow::anyhow!("Server not found: {}", server))?;
        client.call_tool(tool, arguments).await
    }

    /// Call a tool by name, finding the appropriate server automatically
    pub async fn call_tool_auto(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let (server, _) = self
            .find_tool(tool_name)
            .await
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;
        self.call_tool(&server, tool_name, arguments).await
    }
}
