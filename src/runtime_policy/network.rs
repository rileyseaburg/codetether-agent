//! Fail-closed policy for tools controlled by the session network setting.

#[path = "network_governed.rs"]
mod governed;

use super::{DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};

pub(super) fn decision(tool_name: &str, args: &serde_json::Value) -> Option<ToolPolicyDecision> {
    if !governed::check(tool_name, args) || crate::tool::network_access::allowed_for(args) {
        return None;
    }
    Some(ToolPolicyDecision::new(
        ToolPolicyOutcome::Deny,
        DecisionReason::NetworkDisabled,
        ToolKind::Unknown,
    ))
}

pub(super) fn governed(tool: &str, args: &serde_json::Value) -> bool {
    governed::check(tool, args)
}

#[cfg(test)]
#[path = "network_tests.rs"]
mod tests;
