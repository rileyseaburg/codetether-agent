//! Build invalid-tool feedback from registered names.
use crate::*;
pub(crate) async fn execute<H: Host>(agent: &H, name: &str, arguments: Value) -> ToolResult {
    let available_tools = agent.names().iter().map(ToString::to_string).collect();
    codetether_agent_core::execution::invalid::execute(name, arguments, available_tools).await
}
