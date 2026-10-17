//! Invocation decisions with application session and sandbox adapters.

use super::{RuntimeToolPolicy, ToolPolicyDecision};
use serde_json::Value;

pub(super) fn decide(
    policy: &RuntimeToolPolicy,
    tool_name: &str,
    args: &Value,
) -> ToolPolicyDecision {
    codetether_runtime_policy::invocation::decide(
        policy,
        tool_name,
        args,
        || super::session_command::allow(tool_name, args),
        || super::sandbox_preflight::decision(policy, tool_name, args),
    )
}
