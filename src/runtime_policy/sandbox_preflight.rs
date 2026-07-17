//! Approval preflight for unavailable OS sandbox runners.

use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use crate::config::{ApprovalPolicy, SandboxMode};
use serde_json::Value;

pub(super) fn decision(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> Option<ToolPolicyDecision> {
    let command = super::command::value(tool_name, args)?;
    if matches!(policy.approval_policy(), ApprovalPolicy::Never) {
        return None;
    }
    decision_for_state(
        tool_name,
        command,
        policy.sandbox_mode(),
        crate::tool::sandbox::unavailable_reason(),
        crate::tool::sandbox::direct_fallback_env_allowed(),
    )
}

fn decision_for_state(
    tool_name: &str,
    command: &str,
    mode: SandboxMode,
    unavailable: Option<&str>,
    env_allows_direct: bool,
) -> Option<ToolPolicyDecision> {
    if !matches!(tool_name, "bash" | "exec_command")
        || super::command::is_read_only_command(command)
    {
        return None;
    }
    if matches!(mode, SandboxMode::DangerFullAccess) || unavailable.is_none() || env_allows_direct {
        return None;
    }
    Some(ToolPolicyDecision::new(
        ToolPolicyOutcome::RequireApproval,
        DecisionReason::SandboxUnavailable,
        ToolKind::Mutating,
    ))
}

#[cfg(test)]
#[path = "sandbox_preflight_policy_tests.rs"]
mod policy_tests;
#[cfg(test)]
#[path = "sandbox_preflight_tests.rs"]
mod tests;
