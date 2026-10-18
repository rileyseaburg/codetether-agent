//! Preserve native command sandbox, search implementations and write policy.
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use serde_json::Value;
pub(super) struct Native;
impl codetether_mcp_fallback::Host for Native {
    fn execute_bash(args: Value) -> Result<codetether_tool_core::ToolResult> {
        codetether_native_mcp_client::fallback_run_command::execute(args)
    }
    fn blocked(name: &str, args: &Value) -> Option<CallToolResult> {
        codetether_native_mcp_client::tool_policy::blocked(name, args)
    }
}
