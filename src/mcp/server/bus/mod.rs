//! Bus-aware MCP adapters.
//!
//! Each tool and resource owns its registration; execution handlers keep
//! event queries, bridge status and Ralph file reads separate. The parent
//! server invokes these adapters after attaching its live bus bridge.
use super::{
    Arc, BusBridge, CallToolResult, McpServer, ReadResourceResult, ResourceContent,
    ResourceContents, Result, ToolContent, Value, json,
};
mod events;
mod events_handler;
mod prd_resource;
mod progress_resource;
mod ralph_status;
mod ralph_status_handler;
mod recent_resource;
mod status;
mod status_handler;
pub(super) async fn register_tools(server: &McpServer, bridge: Arc<BusBridge>) {
    events::register(server, &bridge).await;
    status::register(server, &bridge).await;
    ralph_status::register(server).await;
    tracing::info!(target: "codetether_agent::mcp::server", "Registered bus-aware MCP tools: bus_events, bus_status, ralph_status");
}
pub(super) async fn register_resources(server: &McpServer, bridge: Arc<BusBridge>) {
    recent_resource::register(server, &bridge).await;
    prd_resource::register(server).await;
    progress_resource::register(server).await;
    tracing::info!(target: "codetether_agent::mcp::server", "Registered bus-aware MCP resources");
}
