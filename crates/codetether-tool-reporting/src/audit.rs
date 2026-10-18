//! Record tool execution with native-equivalent session correlation.
use codetether_audit::{AuditCategory, AuditOutcome, try_audit_log};

/// Record a correlated tool-execution audit entry (no-op without a log).
/// # Arguments
/// Tool identity, session ID, outcome and structured detail.
/// # Returns
/// Completes after optional audit-log recording.
/// # Examples
/// ```no_run
/// # async fn example() {
/// codetether_tool_reporting::record_audit("read", "session", codetether_audit::AuditOutcome::Success, serde_json::json!({})).await;
/// # }
/// ```
pub async fn record_audit(
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
