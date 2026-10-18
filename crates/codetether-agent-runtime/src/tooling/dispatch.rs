//! Native entry point for shared policy-gated tool dispatch.
use crate::{Agent, Host};
use codetether_tool_core::ToolResult;
use serde_json::Value;
impl<H: Host> Agent<H> {
    pub(crate) async fn execute_tool_value(&self, name: &str, arguments: Value) -> ToolResult {
        codetether_agent_tools::execute_value(self, name, arguments).await
    }
}
