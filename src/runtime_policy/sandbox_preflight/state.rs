//! Pure sandbox-availability decision logic.

use crate::config::SandboxMode;
use crate::runtime_policy::{DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};

pub(super) fn for_sandbox(
    tool_name: &str,
    command: &str,
    mode: SandboxMode,
    unavailable: Option<&str>,
    env_allows_direct: bool,
) -> Option<ToolPolicyDecision> {
    if !matches!(tool_name, "bash" | "exec_command")
        || crate::runtime_policy::is_read_only_command(command)
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
