//! Report a missing tool and its correlated audit entry.
use crate::ToolTuple;
use codetether_audit::AuditOutcome;
use serde_json::json;

/// Build the "unknown tool" failure tuple and record an audit entry.
/// # Arguments
/// The missing tool name and session ID.
/// # Returns
/// A failure tuple after optional correlated audit recording.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let result = codetether_tool_reporting::unknown_tool("missing", "session").await;
/// assert!(!result.1);
/// # }
/// ```
pub async fn unknown_tool(tool_name: &str, session_id: &str) -> ToolTuple {
    tracing::warn!(target: "codetether_agent::session::helper::tool_modules::tool_exec::tool_exec_audit",tool = %tool_name, "Tool not found");
    crate::record_audit(
        tool_name,
        session_id,
        AuditOutcome::Failure,
        json!({ "error": "unknown_tool" }),
    )
    .await;
    (format!("Error: Unknown tool '{tool_name}'"), false, None)
}
