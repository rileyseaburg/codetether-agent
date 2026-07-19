//! Result mapping and audit records for one approved tool invocation.

use crate::audit::AuditOutcome;
use crate::tool::Tool;
use std::sync::Arc;

pub(super) async fn run(
    tool: Arc<dyn Tool>,
    input: serde_json::Value,
    name: &str,
    session_id: &str,
    started: std::time::Instant,
) -> super::super::tool_policy::ToolTuple {
    match tool.execute(input).await {
        Ok(result) => success(name, session_id, started, result).await,
        Err(error) => super::tool_exec_failure::run(name, session_id, started, error).await,
    }
}

async fn success(
    name: &str,
    session_id: &str,
    started: std::time::Instant,
    result: crate::tool::ToolResult,
) -> super::super::tool_policy::ToolTuple {
    let outcome = if result.success {
        AuditOutcome::Success
    } else {
        AuditOutcome::Failure
    };
    tracing::info!(tool = %name, success = result.success, "Tool execution completed");
    let detail = super::super::tool_audit_detail::tool_success_detail(elapsed(started), &result);
    super::tool_exec_audit::audit(name, session_id, outcome, detail).await;
    (result.output, result.success, Some(result.metadata))
}

fn elapsed(started: std::time::Instant) -> u64 {
    started.elapsed().as_millis() as u64
}
