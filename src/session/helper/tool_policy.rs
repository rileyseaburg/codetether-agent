//! Runtime policy gate for session helper tool calls.

pub(in crate::session::helper) use codetether_tool_reporting::ToolTuple;
use serde_json::Value;

pub(in crate::session::helper) async fn blocked(
    tool_name: &str,
    args: &Value,
) -> Option<ToolTuple> {
    let result = crate::runtime_policy::evaluate_tool_invocation(tool_name, args).await?;
    Some((result.output, result.success, Some(result.metadata)))
}
