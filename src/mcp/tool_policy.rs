//! Native evaluator and fallback-name bindings for shared MCP enforcement.
use super::types::CallToolResult;
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
struct Native;
impl codetether_mcp_approval::inbound::Evaluator for Native {
    async fn evaluate(name: &str, args: &Value) -> Option<ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(name, args).await
    }
}
pub(super) fn call_registered(
    name: String,
    tool: Arc<dyn Tool>,
    args: Value,
) -> Result<CallToolResult> {
    codetether_mcp_approval::inbound::call_registered::<Native>(name, tool, args)
}
pub(super) fn blocked(name: &str, args: &Value) -> Option<CallToolResult> {
    codetether_mcp_approval::inbound::blocked::<Native>(
        codetether_mcp_fallback::policy::policy_name(name),
        args,
    )
}
