//! Native adapter for shared mutation lease gating.
use serde_json::Value;
pub(in crate::session::helper) async fn blocked(
    tool: &str,
    input: &Value,
) -> Option<codetether_tool_reporting::ToolTuple> {
    codetether_workspace_coordination::blocked(&super::host::NativeHost, tool, input).await
}
#[cfg(test)]
async fn blocked_in_mux(tool: &str, input: &Value) -> Option<codetether_tool_reporting::ToolTuple> {
    codetether_workspace_coordination::blocked_in_mux(&super::host::NativeHost, tool, input).await
}
#[cfg(test)]
#[path = "delegation_tests.rs"]
mod tests;
