//! Invalid tool fallback execution.

use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

/// Render a contextual fallback for an unregistered tool name.
///
/// # Arguments
/// * `name` — Requested tool name.
/// * `arguments` — Original parsed arguments.
/// * `available_tools` — Registered tool names used to suggest alternatives.
/// # Returns
/// The invalid-tool response, or a formatted error if fallback execution fails.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let result = codetether_agent_core::execution::invalid::execute("missing", serde_json::json!({}), vec!["read".into()]).await;
/// assert!(!result.success);
/// # }
/// ```
pub async fn execute(name: &str, arguments: Value, available_tools: Vec<String>) -> ToolResult {
    let invalid_tool =
        codetether_tool_core::InvalidTool::with_context(name.to_string(), available_tools);
    let args = serde_json::json!({ "requested_tool": name, "args": arguments });
    match invalid_tool.execute(args).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Unknown tool: {name}. Error: {error}")),
    }
}
