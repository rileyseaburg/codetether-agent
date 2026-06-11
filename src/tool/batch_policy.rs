//! Runtime policy and dispatch for one nested batch call.

use super::super::{Tool, ToolRegistry, ToolResult, invalid::InvalidTool};
use serde_json::Value;
use std::sync::Arc;

pub(super) async fn execute(
    index: usize,
    tool_id: String,
    args: Value,
    registry: Arc<ToolRegistry>,
) -> (usize, String, ToolResult) {
    if tool_id == "batch" {
        return (
            index,
            tool_id,
            ToolResult::error("Cannot call batch from within batch"),
        );
    }
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(&tool_id, &args).await {
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

async fn invalid(
    index: usize,
    tool_id: String,
    args: Value,
    registry: Arc<ToolRegistry>,
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

#[cfg(test)]
#[path = "batch_policy_tests.rs"]
mod tests;
