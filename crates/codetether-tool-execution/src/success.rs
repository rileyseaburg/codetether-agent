//! Record result status and bounded audit details for completed tools.
use codetether_audit::AuditOutcome;

pub(super) async fn success(
    name: &str,
    session_id: &str,
    started: std::time::Instant,
    result: codetether_tool_core::ToolResult,
) -> codetether_tool_reporting::ToolTuple {
    let outcome = if result.success {
        AuditOutcome::Success
    } else {
        AuditOutcome::Failure
    };
    tracing::info!(target: "codetether_agent::session::helper::tool_modules::tool_exec::tool_exec_run",tool = %name, success = result.success, "Tool execution completed");
    let detail = codetether_tool_reporting::tool_success_detail(elapsed(started), &result);
    codetether_tool_reporting::record_audit(name, session_id, outcome, detail).await;
    (result.output, result.success, Some(result.metadata))
}

fn elapsed(started: std::time::Instant) -> u64 {
    started.elapsed().as_millis() as u64
}
