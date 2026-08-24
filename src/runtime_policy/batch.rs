//! Read-only confinement for nested batch dispatch.

use super::{DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

pub(super) fn decision(tool: &str, args: &Value) -> Option<ToolPolicyDecision> {
    if tool != "batch" {
        return None;
    }
    let safe = args
        .get("calls")
        .and_then(Value::as_array)
        .is_some_and(|calls| !calls.is_empty() && calls.iter().all(read_only));
    let (outcome, kind) = if safe {
        (ToolPolicyOutcome::Allow, ToolKind::ReadOnly)
    } else {
        (ToolPolicyOutcome::Deny, ToolKind::Mutating)
    };
    Some(ToolPolicyDecision::new(
        outcome,
        DecisionReason::MutatingTool,
        kind,
    ))
}

fn read_only(call: &Value) -> bool {
    call.get("tool")
        .or_else(|| call.get("name"))
        .and_then(Value::as_str)
        .is_some_and(crate::tool::readonly::is_read_only)
}
