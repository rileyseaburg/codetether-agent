//! Attach shared MCP bus-monitoring registrations to the native catalog.
use super::{BusBridge, McpServer};
use std::sync::Arc;
pub(super) async fn register_tools(server: &McpServer, bridge: Arc<BusBridge>) {
    codetether_mcp_bus::register_tools(&server.inner.catalog, bridge).await;
}
pub(super) async fn register_resources(server: &McpServer, bridge: Arc<BusBridge>) {
    codetether_mcp_bus::register_resources(&server.inner.catalog, bridge).await;
}
