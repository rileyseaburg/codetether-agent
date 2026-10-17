//! MCP server catalog register_tool operations.
use super::*;
impl ServerCatalog {
    /// Register a tool
    pub async fn register_tool(
        &self,
        name: &str,
        description: &str,
        input_schema: Value,
        handler: McpToolHandler,
    ) {
        // Store tool metadata
        let metadata = ToolMetadata::new(
            name.to_string(),
            Some(description.to_string()),
            input_schema.clone(),
        );

        let mut metadata_map = self.metadata.write().await;
        metadata_map.insert(name.to_string(), metadata);
        drop(metadata_map);

        let mut tools = self.tools.write().await;
        tools.insert(name.to_string(), handler);

        tracing::debug!(target: "codetether_agent::mcp::server", "Registered MCP tool: {}", name);
    }
}
