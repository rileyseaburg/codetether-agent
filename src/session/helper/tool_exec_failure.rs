//! Error mapping and audit records for a failed tool invocation.

use crate::audit::AuditOutcome;

#[path = "tool_exec_error_text.rs"]
mod text;

pub(super) async fn run(
    name: &str,
    session_id: &str,
    started: std::time::Instant,
    error: anyhow::Error,
) -> super::super::tool_policy::ToolTuple {
    let diagnostic = text::diagnostic(&error);
    tracing::warn!(tool = %name, error = %diagnostic, "Tool execution failed");
    let elapsed = started.elapsed().as_millis() as u64;
    let detail = super::super::tool_audit_detail::tool_failure_detail(elapsed, &diagnostic);
    super::tool_exec_audit::audit(name, session_id, AuditOutcome::Failure, detail).await;
    (format!("Error: {diagnostic}"), false, None)
}
