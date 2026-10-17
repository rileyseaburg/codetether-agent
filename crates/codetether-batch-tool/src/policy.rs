//! Runtime policy and dispatch for one nested batch call.

use super::Registry;
use codetether_tool_core::{InvalidTool, Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;

/// Check policy and dispatch one indexed batch call.
///
/// # Arguments
/// * `index` - Position of this call in the original batch.
/// * `tool_id` - Requested tool identifier.
/// * `args` - Nested tool arguments.
/// * `registry` - Registry providing lookup and policy evaluation.
/// # Returns
/// The original index, tool identifier and success or error result.
/// # Examples
/// ```rust,no_run
/// # async fn example<R: codetether_batch_tool::Registry>(registry: std::sync::Arc<R>) {
/// let _result = codetether_batch_tool::policy::execute(0, "read".into(),
///     serde_json::json!({"path":"file.txt"}), registry).await;
/// # }
/// ```
pub async fn execute<R: Registry>(
    index: usize,
    tool_id: String,
    args: Value,
    registry: Arc<R>,
) -> (usize, String, ToolResult) {
    if tool_id == "batch" {
        return (
            index,
            tool_id,
            ToolResult::error("Cannot call batch from within batch"),
        );
    }
    if let Some(blocked) = registry.evaluate_tool_invocation(&tool_id, &args).await {
        return (index, tool_id, blocked);
    }
    match registry.get(&tool_id) {
        Some(tool) => match tool.execute(args).await {
            Ok(result) => (index, tool_id, result),
            Err(error) => (index, tool_id, ToolResult::error(format!("Error: {error}"))),
        },
        None => invalid(index, tool_id, args, registry).await,
    }
}

async fn invalid<R: Registry>(
    index: usize,
    tool_id: String,
    args: Value,
    registry: Arc<R>,
) -> (usize, String, ToolResult) {
    let available_tools = registry.list().iter().map(|s| s.to_string()).collect();
    let invalid_tool = InvalidTool::with_context(tool_id.clone(), available_tools);
    let invalid_args = serde_json::json!({ "requested_tool": tool_id.clone(), "args": args });
    match invalid_tool.execute(invalid_args).await {
        Ok(result) => (index, tool_id.clone(), result),
        Err(error) => (
            index,
            tool_id.clone(),
            ToolResult::error(format!("Unknown tool: {tool_id}. Error: {error}")),
        ),
    }
}
