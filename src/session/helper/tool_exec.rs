//! Single tool execution with audit logging and progress wiring.
//!
//! Extracted from [`super::prompt_events`] to keep that loop within the file
//! budget. Owns the registry lookup, policy gate, progress-id injection, and
//! success/failure audit records for one tool call. Audit/progress helpers live
//! in [`tool_exec_audit`].

#[path = "tool_exec_audit.rs"]
mod tool_exec_audit;

use tokio::sync::mpsc;

use crate::audit::AuditOutcome;
use crate::session::SessionEvent;
use crate::tool::ToolRegistry;

use super::tool_audit_detail::{tool_failure_detail, tool_success_detail};
use tool_exec_audit::{audit, progress_input, unknown_tool};

/// Execute one tool and return its `(output, success, metadata)` tuple.
pub(in crate::session::helper) async fn execute_tool(
    tool_registry: &ToolRegistry,
    tool_name: &str,
    exec_input: &serde_json::Value,
    session_id: &str,
    exec_start: std::time::Instant,
    progress: Option<(&mpsc::Sender<SessionEvent>, &str)>,
) -> super::tool_policy::ToolTuple {
    let Some(tool) = tool_registry.get(tool_name) else {
        return unknown_tool(tool_name, session_id).await;
    };
    if let Some(blocked) = super::tool_policy::blocked(tool_name, exec_input).await {
        return blocked;
    }
    let _guard = progress.map(|(tx, id)| crate::tool::progress::register(id, tool_name, tx));
    let exec_input = progress_input(exec_input, progress.map(|(_, id)| id));
    let ms = || exec_start.elapsed().as_millis() as u64;
    match tool.execute(exec_input).await {
        Ok(result) => {
            let outcome = if result.success {
                AuditOutcome::Success
            } else {
                AuditOutcome::Failure
            };
            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
            audit(
                tool_name,
                session_id,
                outcome,
                tool_success_detail(ms(), &result),
            )
            .await;
            (result.output, result.success, Some(result.metadata))
        }
        Err(e) => {
            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
            let detail = tool_failure_detail(ms(), &e.to_string());
            audit(tool_name, session_id, AuditOutcome::Failure, detail).await;
            (format!("Error: {e}"), false, None)
        }
    }
}
