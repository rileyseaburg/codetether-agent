//! Collect registered tool names for the shared invalid-tool fallback.

use crate::agent::Agent;
use crate::tool::ToolResult;
use serde_json::Value;

pub(super) async fn execute(agent: &Agent, name: &str, arguments: Value) -> ToolResult {
    let available_tools = agent.tools.list().iter().map(ToString::to_string).collect();
    codetether_agent_core::execution::invalid::execute(name, arguments, available_tools).await
}
