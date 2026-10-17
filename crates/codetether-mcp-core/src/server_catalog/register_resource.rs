//! MCP server catalog register_resource operations.
use super::*;
impl ServerCatalog {
    /// Register a resource
    pub async fn register_resource(
        &self,
        uri: &str,
        name: &str,
        description: &str,
        mime_type: Option<&str>,
        handler: McpResourceHandler,
    ) {
        // Store resource metadata
        let metadata = ResourceMetadata::new(
            uri.to_string(),
            name.to_string(),
            Some(description.to_string()),
            mime_type.map(|s| s.to_string()),
        );

        let mut metadata_map = self.resource_metadata.write().await;
        metadata_map.insert(uri.to_string(), metadata);
        drop(metadata_map);

        let mut resources = self.resources.write().await;
        resources.insert(uri.to_string(), handler);

        tracing::debug!(target: "codetether_agent::mcp::server", "Registered MCP resource: {}", uri);
    }
}
