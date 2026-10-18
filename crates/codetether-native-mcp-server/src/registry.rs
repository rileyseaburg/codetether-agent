//! Native executable-tool policy for shared MCP registration.
mod host;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_mcp_core::server_catalog::ServerCatalog;
use std::sync::Arc;
pub(super) async fn register<T: Application>(
    catalog: &ServerCatalog,
    registry: &Arc<ToolRegistry<T>>,
) {
    codetether_mcp_server::registry::register(catalog, &host::Registry(registry.as_ref())).await;
}
