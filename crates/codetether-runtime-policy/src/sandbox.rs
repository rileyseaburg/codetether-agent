//! Sandbox approval decisions from caller-supplied availability state.

use crate::{DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use codetether_config::SandboxMode;

/// Decide whether unavailable sandboxing requires approval for a shell command.
///
/// # Arguments
/// * `tool_name` — Tool identifier.
/// * `command` — Shell command text.
/// * `mode` — Effective sandbox mode.
/// * `unavailable` — Reason the sandbox is unavailable, if any.
/// * `env_allows_direct` — Whether the host explicitly permits direct fallback.
/// # Returns
/// An approval requirement, or `None` when this check does not apply.
/// # Examples
/// ```
/// use codetether_config::SandboxMode;
/// assert!(codetether_runtime_policy::sandbox::decision_for_state("bash", "pwd", SandboxMode::WorkspaceWrite, Some("unavailable"), false).is_none());
/// ```

pub fn decision_for_state(
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
