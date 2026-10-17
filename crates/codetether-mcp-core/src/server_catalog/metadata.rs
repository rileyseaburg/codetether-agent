//! MCP server catalog metadata operations.
use super::*;
impl ServerCatalog {
    /// Get tool metadata by name
    pub async fn get_tool_metadata(&self, name: &str) -> Option<ToolMetadata> {
        let metadata = self.metadata.read().await;
        metadata.get(name).cloned()
    }

    /// Get all tool metadata
    pub async fn get_all_tool_metadata(&self) -> Vec<ToolMetadata> {
        let metadata = self.metadata.read().await;
        metadata.values().cloned().collect()
    }

    /// Get resource metadata by URI
    pub async fn get_resource_metadata(&self, uri: &str) -> Option<ResourceMetadata> {
        let metadata = self.resource_metadata.read().await;
        metadata.get(uri).cloned()
    }

    /// Get all resource metadata
    pub async fn get_all_resource_metadata(&self) -> Vec<ResourceMetadata> {
        let metadata = self.resource_metadata.read().await;
        metadata.values().cloned().collect()
    }
}
