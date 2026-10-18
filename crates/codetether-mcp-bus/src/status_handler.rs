//! Execution of the status MCP tool.
use super::*;
pub(super) fn handler(b: Arc<BusBridge>) -> codetether_mcp_core::server_catalog::McpToolHandler {
    Arc::new(move |_args| {
        let status = b.status();
        let buffer_len = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(b.buffer_len())
        });

        let text = serde_json::to_string_pretty(&json!({
            "connected": status.connected,
            "total_received": status.total_received,
            "buffer_used": buffer_len,
            "buffer_capacity": status.buffer_capacity,
            "bus_url": status.bus_url,
        }))
        .unwrap_or_default();

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text }],
            is_error: false,
        })
    })
}
