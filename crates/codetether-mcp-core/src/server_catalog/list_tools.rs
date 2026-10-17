//! MCP server catalog list_tools operations.
use super::*;
impl ServerCatalog {
    /// Handle list tools request - reads from ToolMetadata registry
    pub async fn handle_list_tools(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        // Read tools from the metadata registry instead of hardcoded list
        let metadata = self.metadata.read().await;

        let tool_list: Vec<McpTool> = metadata
            .values()
            .map(|tm| McpTool {
                name: tm.name.clone(),
                description: tm.description.clone(),
                input_schema: tm.input_schema.clone(),
            })
            .collect();

        let result = ListToolsResult {
            tools: tool_list,
            next_cursor: None,
        };

        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
}
