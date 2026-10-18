//! Shared native runtime-policy bindings for inbound MCP tools and processes.
use codetether_tool_core::ToolResult;
use serde_json::Value;

pub(super) struct Native;
impl codetether_mcp_approval::inbound::Evaluator for Native {
    async fn evaluate(name: &str, args: &Value) -> Option<ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(name, args).await
    }
}
impl codetether_mcp_approval::subprocess::Host for Native {
    fn approved(name: &str, args: &Value) -> bool {
        codetether_native_runtime_policy::approved_invocation(name, args)
    }
}
