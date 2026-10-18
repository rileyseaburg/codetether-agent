//! Native catalog and registry bindings for shared MCP setup.
use super::*;
impl McpServer {
    /// Setup default tools (public, for CLI use)
    pub async fn setup_tools_public(&self) {
        self.setup_tools().await;
    }

    /// Register the configured tools through shared setup sequencing.
    pub(super) async fn setup_tools(&self) {
        codetether_mcp_server::setup::run(self).await;
    }
}

impl codetether_mcp_server::setup::Host for McpServer {
    type Registry = Arc<ToolRegistry>;

    fn catalog(&self) -> &ServerCatalog {
        &self.catalog
    }

    fn registry(&self) -> Option<&Self::Registry> {
        self.tool_registry.as_ref()
    }

    async fn register_registry(&self, registry: &Self::Registry) {
        self.register_registry_tools(registry).await;
    }

    async fn register_fallback(&self) {
        fallback::register(self).await;
    }
}
