//! Structured tool failures produced by the authoritative mutation gate.

use crate::mux::lease::WorktreeLease;

pub(super) fn conflict(
    tool: &str,
    conflicts: &[WorktreeLease],
) -> super::super::tool_policy::ToolTuple {
    result(
        "WORKTREE_LEASE_CONFLICT",
        tool,
        "Mutation blocked because another mux agent owns an overlapping path.",
        serde_json::json!({ "conflicts": conflicts }),
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
