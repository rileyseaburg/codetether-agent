//! Bind the native registry to shared MCP tool selection and registration.
mod host;
use super::{McpServer, ToolRegistry};
use std::sync::Arc;
impl McpServer {
    pub(super) async fn register_registry_tools(&self, registry: &Arc<ToolRegistry>) {
        codetether_mcp_server::registry::register(
            &self.inner.catalog,
            &host::Registry(registry.as_ref()),
        )
        .await;
    }
}
