//! Runtime policy gate for session helper tool calls.

use serde_json::Value;
use std::collections::HashMap;

pub(in crate::session::helper) type ToolTuple = (String, bool, Option<HashMap<String, Value>>);

pub(in crate::session::helper) async fn blocked(
    tool_name: &str,
    args: &Value,
) -> Option<ToolTuple> {
    let result = crate::runtime_policy::evaluate_tool_invocation(tool_name, args).await?;
    Some((result.output, result.success, Some(result.metadata)))
}
