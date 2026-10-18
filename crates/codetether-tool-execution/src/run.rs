//! Invoke one resolved tool and preserve complete failure reporting.
use codetether_tool_core::Tool;
use std::sync::Arc;

pub(super) async fn run(
    tool: Arc<dyn Tool>,
    input: serde_json::Value,
    name: &str,
    session_id: &str,
    started: std::time::Instant,
) -> codetether_tool_reporting::ToolTuple {
    match tool.execute(input).await {
        Ok(result) => crate::success::success(name, session_id, started, result).await,
        Err(error) => codetether_tool_reporting::failure(name, session_id, started, error).await,
    }
}
