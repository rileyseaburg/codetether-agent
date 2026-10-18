//! Execution of the events MCP tool.
use super::*;
pub(super) fn handler(b: Arc<BusBridge>) -> codetether_mcp_core::server_catalog::McpToolHandler {
    Arc::new(move |args| {
        let topic_filter = args
            .get("topic_filter")
            .and_then(|v| v.as_str())
            .map(String::from);
        let limit = args
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(50)
            .min(500) as usize;

        let b = Arc::clone(&b);
        let events = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { b.recent_events(topic_filter.as_deref(), limit, None).await })
        });

        let text = serde_json::to_string_pretty(&events).unwrap_or_else(|_| "[]".to_string());

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text }],
            is_error: false,
        })
    })
}
