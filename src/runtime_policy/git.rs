//! Operation-aware policy classification for the structured Git tool.

use super::{RuntimeToolPolicy, ToolKind, ToolPolicyDecision};
use serde_json::Value;

pub(super) fn decision(
    policy: &RuntimeToolPolicy,
    tool: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    if tool != "git" {
        return None;
    }
    let op = args.get("op").and_then(Value::as_str).unwrap_or("");
    let kind = if op == "commit" {
        ToolKind::Mutating
    } else {
        ToolKind::ReadOnly
    };
    Some(policy.decide_as(tool, kind))
}
