//! Coordinator availability and protocol failures for mutation attempts.

pub(super) fn unavailable(
    tool: &str,
    error: &anyhow::Error,
) -> super::super::tool_policy::ToolTuple {
    super::gate_error::result(
        "WORKTREE_COORDINATOR_UNAVAILABLE",
        tool,
        "Mutation blocked because the inherited mux coordinator is unavailable.",
        serde_json::json!({ "cause": error.to_string() }),
    )
}

pub(super) fn invalid(tool: &str) -> super::super::tool_policy::ToolTuple {
    super::gate_error::result(
        "WORKTREE_COORDINATOR_PROTOCOL",
        tool,
        "Mux returned an invalid lease response.",
        serde_json::json!({}),
    )
}

pub(super) fn missing(tool: &str) -> super::super::tool_policy::ToolTuple {
    super::gate_error::result(
        "WORKTREE_COORDINATOR_MISSING",
        tool,
        "Mux identity disappeared before mutation.",
        serde_json::json!({}),
    )
}
