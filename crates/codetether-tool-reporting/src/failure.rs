//! Error mapping and audit records for a failed tool invocation.

use codetether_audit::AuditOutcome;

use crate::diagnostic;

/// # Arguments
/// Tool/session IDs, start instant and the complete error chain.
/// # Returns
/// A failure tuple after logging and optional audit recording.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let result = codetether_tool_reporting::failure("read", "session", std::time::Instant::now(), anyhow::anyhow!("failed")).await;
/// assert!(!result.1);
/// # }
/// ```
pub async fn failure(
    name: &str,
    session_id: &str,
    started: std::time::Instant,
    error: anyhow::Error,
) -> crate::ToolTuple {
    let diagnostic = diagnostic(&error);
    tracing::warn!(target: "codetether_agent::session::helper::tool_modules::tool_exec::tool_exec_failure",tool = %name, error = %diagnostic, "Tool execution failed");
    let elapsed = started.elapsed().as_millis() as u64;
    let detail = crate::tool_failure_detail(elapsed, &diagnostic);
    crate::record_audit(name, session_id, AuditOutcome::Failure, detail).await;
    (format!("Error: {diagnostic}"), false, None)
}
