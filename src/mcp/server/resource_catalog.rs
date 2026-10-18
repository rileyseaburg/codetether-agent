//! Public resource registration and metadata access.
use super::*;
impl McpServer {
    /// Register a resource
    pub async fn register_resource(
        &self,
        uri: &str,
        name: &str,
        description: &str,
        mime_type: Option<&str>,
        handler: McpResourceHandler,
    ) {
        self.inner
            .catalog
            .register_resource(uri, name, description, mime_type, handler)
            .await
    }

    /// Get resource metadata by URI
    pub async fn get_resource_metadata(&self, uri: &str) -> Option<ResourceMetadata> {
        self.inner.catalog.get_resource_metadata(uri).await
    }

    /// Get all resource metadata
    pub async fn get_all_resource_metadata(&self) -> Vec<ResourceMetadata> {
        self.inner.catalog.get_all_resource_metadata().await
    }
}
