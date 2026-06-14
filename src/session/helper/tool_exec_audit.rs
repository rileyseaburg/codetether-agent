//! Audit + progress helpers for single tool execution.
//!
//! Split from [`super::tool_exec`] to honor the 50-line file budget.

use serde_json::json;

use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};

/// Tuple returned by tool execution: `(output, success, metadata)`.
pub(super) type ToolTuple = crate::session::helper::tool_policy::ToolTuple;

/// Build the "unknown tool" failure tuple and record an audit entry.
pub(super) async fn unknown_tool(tool_name: &str, session_id: &str) -> ToolTuple {
    tracing::warn!(tool = %tool_name, "Tool not found");
    audit(
        tool_name,
        session_id,
        AuditOutcome::Failure,
        json!({ "error": "unknown_tool" }),
    )
    .await;
    (format!("Error: Unknown tool '{tool_name}'"), false, None)
}

/// Record a correlated tool-execution audit entry (no-op without a log).
pub(super) async fn audit(
    tool_name: &str,
    session_id: &str,
    outcome: AuditOutcome,
    detail: serde_json::Value,
) {
    let Some(audit) = try_audit_log() else { return };
    audit
        .log_with_correlation(
            AuditCategory::ToolExecution,
            format!("tool:{tool_name}"),
            outcome,
            None,
            Some(detail),
            None,
            None,
            None,
            Some(session_id.to_string()),
        )
        .await;
}

/// Inject the progress `_tool_call_id` into a tool's input object.
pub(super) fn progress_input(input: &serde_json::Value, id: Option<&str>) -> serde_json::Value {
    let Some(id) = id else { return input.clone() };
    let mut input = input.clone();
    if let serde_json::Value::Object(map) = &mut input {
        map.insert(
            "_tool_call_id".into(),
            serde_json::Value::String(id.to_string()),
        );
    }
    input
}
