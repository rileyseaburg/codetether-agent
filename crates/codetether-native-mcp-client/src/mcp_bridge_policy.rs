//! Application-bound adapters for the existing bridge approval tests.
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn blocked(
    command: &str,
    args: &[&str],
    approval_id: Option<&str>,
) -> Option<ToolResult> {
    codetether_mcp_bridge::policy::blocked::<super::ApplicationHost>(command, args, approval_id)
        .await
}
use codetether_mcp_bridge::policy::policy_args;
#[path = "mcp_bridge_policy_tests.rs"]
mod tests;
