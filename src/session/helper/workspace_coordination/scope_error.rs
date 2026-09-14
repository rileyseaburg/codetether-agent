//! Rejection returned when a coordinator refuses a whole-workspace claim.

pub(super) fn result(tool: &str) -> super::super::tool_policy::ToolTuple {
    super::gate_error::result(
        "WORKSPACE_LEASE_FORBIDDEN",
        tool,
        "Mux sessions cannot own a workspace. Coordinate only explicit file or bounded subtree paths.",
        serde_json::json!({}),
    )
}
