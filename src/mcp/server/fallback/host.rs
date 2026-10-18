//! Preserve native command sandbox, search implementations and write policy.
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use serde_json::Value;
pub(super) struct Native;
impl codetether_mcp_fallback::Host for Native {
    fn run_command(args: Value) -> Result<CallToolResult> {
        crate::mcp::fallback_run_command::call(args)
    }
    fn search_files(args: Value) -> Result<CallToolResult> {
        crate::mcp::fallback_search::search_files(args)
    }
    fn grep_search(args: Value) -> Result<CallToolResult> {
        crate::mcp::fallback_search::grep_search(args)
    }
    fn blocked(name: &str, args: &Value) -> Option<CallToolResult> {
        crate::mcp::tool_policy::blocked(name, args)
    }
}
