//! Bus monitoring tools and Ralph status resources for MCP catalogs.
//!
//! [register_tools] installs event/status queries and Ralph status reads.
//! [register_resources] exposes recent events and the current Ralph files.
//! Pass an existing [ServerCatalog] and live [BusBridge] after attaching the bus.
use anyhow::Result;
use codetether_bus_http::BusBridge;
use codetether_mcp_core::{
    server_catalog::ServerCatalog,
    types::{CallToolResult, ReadResourceResult, ResourceContent, ResourceContents, ToolContent},
};
use serde_json::{Value, json};
use std::sync::Arc;
mod attachment;
mod events;
mod events_handler;
mod prd_resource;
mod progress_resource;
mod ralph_status;
mod ralph_status_handler;
mod recent_resource;
mod status;
mod status_handler;
pub use attachment::attach;
/// Register bus-event, bridge-status and Ralph-status tools.
/// # Arguments
/// * `server` - Catalog receiving handlers and schemas.
/// * `bridge` - Existing live bridge queried by the handlers.
/// # Returns
/// Registers the three tools in their original order.
/// # Examples
/// ```no_run
/// # async fn example(bridge: std::sync::Arc<codetether_bus_http::BusBridge>) {
/// let catalog = codetether_mcp_core::server_catalog::ServerCatalog::new();
/// codetether_mcp_bus::register_tools(&catalog, bridge).await;
/// # }
/// ```
pub async fn register_tools(server: &ServerCatalog, bridge: Arc<BusBridge>) {
    events::register(server, &bridge).await;
    status::register(server, &bridge).await;
    ralph_status::register(server).await;
    tracing::info!(target: "codetether_agent::mcp::server", "Registered bus-aware MCP tools: bus_events, bus_status, ralph_status");
}
/// Register recent bus events, PRD JSON and Ralph progress resources.
/// # Arguments
/// * `server` - Catalog receiving resource handlers and metadata.
/// * `bridge` - Existing live bridge used for recent events.
/// # Returns
/// Registers the three resources in their original order.
/// # Examples
/// ```no_run
/// # async fn example(bridge: std::sync::Arc<codetether_bus_http::BusBridge>) {
/// let catalog = codetether_mcp_core::server_catalog::ServerCatalog::new();
/// codetether_mcp_bus::register_resources(&catalog, bridge).await;
/// # }
/// ```
pub async fn register_resources(server: &ServerCatalog, bridge: Arc<BusBridge>) {
    recent_resource::register(server, &bridge).await;
    prd_resource::register(server).await;
    progress_resource::register(server).await;
    tracing::info!(target: "codetether_agent::mcp::server", "Registered bus-aware MCP resources");
}
