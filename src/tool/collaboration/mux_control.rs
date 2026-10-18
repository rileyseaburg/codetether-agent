//! Native projection reader and registry binding for the shared mux tool.
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
pub(super) struct MuxControlTool;
struct Native;
#[async_trait]
impl codetether_mux_tool::Host for Native {
    async fn read_agent_output(name: &str) -> Result<Option<String>> {
        crate::mux::control::read_agent_output(name).await
    }
}
#[async_trait]
impl Tool for MuxControlTool {
    fn id(&self) -> &str {
        codetether_mux_tool::ID
    }
    fn name(&self) -> &str {
        codetether_mux_tool::NAME
    }
    fn description(&self) -> &str {
        codetether_mux_tool::DESCRIPTION
    }
    fn parameters(&self) -> Value {
        codetether_mux_tool::parameters()
    }
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        codetether_mux_tool::execute::<Native>(input).await
    }
}
#[cfg(test)]
#[path = "mux_control/tests.rs"]
mod tests;
