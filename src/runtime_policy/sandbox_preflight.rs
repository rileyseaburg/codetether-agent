//! Approval preflight for unavailable OS sandbox runners.

use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use crate::config::ApprovalPolicy;
use serde_json::Value;

#[path = "sandbox_preflight/state.rs"]
mod state;

pub(super) fn decision(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    let command = super::command::value(tool_name, args)?;
    if matches!(policy.approval_policy(), ApprovalPolicy::Never) {
        return None;
    }
    state::for_sandbox(
        tool_name,
        command,
        policy.sandbox_mode(),
        crate::tool::sandbox::unavailable_reason(),
        crate::tool::sandbox::direct_fallback_env_allowed(),
    )
}

pub(super) fn escalation(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    let escalated = tool_name == "exec_command"
        && args.get("sandbox_permissions").and_then(Value::as_str) == Some("require_escalated");
    if !escalated {
        return None;
    }
    let (outcome, reason) = match policy.approval_policy() {
        ApprovalPolicy::Never => (ToolPolicyOutcome::Deny, DecisionReason::ApprovalUnavailable),
        ApprovalPolicy::Untrusted | ApprovalPolicy::OnFailure | ApprovalPolicy::OnRequest => (
            ToolPolicyOutcome::RequireApproval,
            DecisionReason::SandboxEscalation,
        ),
    };
    Some(ToolPolicyDecision::new(outcome, reason, ToolKind::Mutating))
}

#[cfg(test)]
#[path = "sandbox_preflight_policy_tests.rs"]
mod policy_tests;
#[cfg(test)]
#[path = "sandbox_preflight_tests.rs"]
mod tests;
