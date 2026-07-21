//! Structured tool failures produced by the authoritative mutation gate.

use crate::mux::lease::WorktreeLease;

pub(super) fn conflict(
    tool: &str,
    conflicts: &[WorktreeLease],
    waited_ms: u64,
    retry_after_ms: u64,
) -> super::super::tool_policy::ToolTuple {
    let waited_seconds = waited_ms / 1_000;
    result(
        "WORKTREE_LEASE_WAIT_TIMEOUT",
        tool,
        &format!(
            "Coordinator already waited {waited_seconds}s for the overlapping lease. Do not call wait_agent or ask another agent to perform the mutation; continue non-conflicting work or retry it later."
        ),
        serde_json::json!({
            "waited_ms": waited_ms,
            "retry_after_ms": retry_after_ms,
            "conflicts": conflicts
        }),
    )
}

pub(super) fn delegation(tool: &str, action: &str) -> super::super::tool_policy::ToolTuple {
    result(
        "MUX_AGENT_DELEGATION_FORBIDDEN",
        tool,
        "Mux-managed agents cannot create or delegate work to child agents. Continue locally or return control to the user. Steering an existing mux session with agent message remains available.",
        serde_json::json!({ "action": action }),
    )
}

pub(super) fn result(
    code: &str,
    tool: &str,
    message: &str,
    detail: serde_json::Value,
) -> super::super::tool_policy::ToolTuple {
    let result = crate::tool::ToolResult::structured_error(code, tool, message, None, Some(detail));
    (result.output, false, Some(result.metadata))
}
