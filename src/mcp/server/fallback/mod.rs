//! Native fallback execution policy for shared MCP registration.
mod host;
use codetether_mcp_core::server_catalog::ServerCatalog;
pub(super) async fn register(catalog: &ServerCatalog) {
    codetether_mcp_fallback::register::<host::Native>(catalog).await;
}
