//! Select registry or fallback tool setup.
use super::*;
impl McpServer {
    /// Setup default tools (public, for CLI use)
    pub async fn setup_tools_public(&self) {
        self.setup_tools().await;
    }

    /// Setup default tools
    pub(super) async fn setup_tools(&self) {
        // If a ToolRegistry was provided, bridge ALL its tools into MCP
        if let Some(registry) = &self.tool_registry {
            self.register_registry_tools(registry).await;
            tracing::info!(target: "codetether_agent::mcp::server",
                "Registered {} MCP tools from ToolRegistry",
                self.catalog.tools().read().await.len()
            );
            return;
        }

        // Fallback: register basic hardcoded tools when no registry is available
        self.register_fallback_tools().await;
        tracing::info!(target: "codetether_agent::mcp::server",
            "Registered {} MCP tools (fallback)",
            self.catalog.tools().read().await.len()
        );
    }
    pub(super) async fn register_fallback_tools(&self) {
        fallback::register(self).await;
    }
}
