//! MCP server catalog list_resources operations.
use super::*;
impl ServerCatalog {
    /// Handle list resources request — reads from registered resource metadata
    pub async fn handle_list_resources(
        &self,
        _params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        let metadata = self.resource_metadata.read().await;
        let resource_list: Vec<McpResource> = metadata
            .values()
            .map(|rm| McpResource {
                uri: rm.uri.clone(),
                name: rm.name.clone(),
                description: rm.description.clone(),
                mime_type: rm.mime_type.clone(),
            })
            .collect();

        let result = ListResourcesResult {
            resources: resource_list,
            next_cursor: None,
        };

        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
}
