//! Registered tool execution wrapper.

use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;

/// Execute a registered tool and bound its result to the configured output budget.
///
/// # Arguments
/// * `tool` — Selected tool implementation.
/// * `arguments` — Parsed arguments supplied to the tool.
/// # Returns
/// Tool output or a formatted execution error, both subject to the output budget.
/// # Examples
/// ```no_run
/// # async fn example(tool: std::sync::Arc<dyn codetether_tool_core::Tool>) {
/// let result = codetether_agent_core::execution::registered::execute(tool, serde_json::json!({})).await;
/// # }
/// ```
pub async fn execute(tool: Arc<dyn Tool>, arguments: Value) -> ToolResult {
    let result = match tool.execute(arguments).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Tool execution failed: {error}")),
    };
    result.truncate_to(codetether_tool_core::tool_output_budget())
}
