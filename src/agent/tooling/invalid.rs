//! Invalid tool fallback execution.

use crate::agent::Agent;
use crate::tool::{Tool, ToolResult};
use serde_json::Value;

pub(super) async fn execute(agent: &Agent, name: &str, arguments: Value) -> ToolResult {
    let available_tools = agent.tools.list().iter().map(ToString::to_string).collect();
    let invalid_tool =
        crate::tool::invalid::InvalidTool::with_context(name.to_string(), available_tools);
    let args = serde_json::json!({ "requested_tool": name, "args": arguments });
    match invalid_tool.execute(args).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Unknown tool: {name}. Error: {error}")),
    }
}
