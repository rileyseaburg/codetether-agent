//! Approval preflight for unavailable OS sandbox runners.

use super::{RuntimeToolPolicy, ToolPolicyDecision};
use crate::config::ApprovalPolicy;
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
        codetether_sandbox_runtime::unavailable_reason(),
        codetether_sandbox_runtime::direct_fallback_env_allowed(),
    )
}
use codetether_runtime_policy::sandbox::decision_for_state;

#[cfg(test)]
#[path = "sandbox_preflight_policy_tests.rs"]
mod policy_tests;
#[cfg(test)]
#[path = "sandbox_preflight_tests.rs"]
mod tests;
