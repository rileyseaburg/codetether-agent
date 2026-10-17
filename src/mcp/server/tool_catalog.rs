//! Public tool catalog access and direct invocation.
use super::*;
impl McpServer {
    /// Register a tool
    pub async fn register_tool(
        &self,
        name: &str,
        description: &str,
        input_schema: Value,
        handler: McpToolHandler,
    ) {
        self.catalog
            .register_tool(name, description, input_schema, handler)
            .await
    }

    /// Get tool metadata by name
    pub async fn get_tool_metadata(&self, name: &str) -> Option<ToolMetadata> {
        self.catalog.get_tool_metadata(name).await
    }

    /// Get all tool metadata
    pub async fn get_all_tool_metadata(&self) -> Vec<ToolMetadata> {
        self.catalog.get_all_tool_metadata().await
    }

    /// Call a tool directly without going through the transport
    pub async fn call_tool_direct(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        self.catalog.call_tool_direct(name, arguments).await
    }
}
