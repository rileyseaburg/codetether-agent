//! Approval boundary for input sent to persistent command sessions.

use super::{RuntimeToolPolicy, ToolKind, ToolPolicyDecision};
use serde_json::Value;

pub(super) fn decision(
    policy: &RuntimeToolPolicy,
    tool: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    if tool != "write_stdin"
        || args
            .get("chars")
            .and_then(Value::as_str)
            .unwrap_or("")
            .is_empty()
    {
        return None;
    }
    Some(policy.decide_as(tool, ToolKind::SessionTransport))
}
