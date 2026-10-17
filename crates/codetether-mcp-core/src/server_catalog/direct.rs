//! MCP server catalog direct operations.
use super::*;
impl ServerCatalog {
    /// Call a tool directly without going through the transport
    pub async fn call_tool_direct(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        let tools = self.tools.read().await;
        let handler = tools
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?
            .clone();
        drop(tools);
        handler(arguments)
    }
}
