//! Preserve native command sandbox, search implementations and write policy.
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use serde_json::Value;
pub(super) struct Native;
impl codetether_mcp_fallback::Host for Native {
    fn execute_bash(args: Value) -> Result<crate::tool::ToolResult> {
        crate::mcp::fallback_run_command::execute(args)
    }
    fn blocked(name: &str, args: &Value) -> Option<CallToolResult> {
        crate::mcp::tool_policy::blocked(name, args)
    }
}
