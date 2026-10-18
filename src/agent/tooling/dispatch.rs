//! Native entry point for shared policy-gated tool dispatch.
use crate::{agent::Agent, tool::ToolResult};
use serde_json::Value;
impl Agent {
    pub(crate) async fn execute_tool_value(&self, name: &str, arguments: Value) -> ToolResult {
        codetether_agent_tools::execute_value(self, name, arguments).await
    }
}
