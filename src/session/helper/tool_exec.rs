//! Single tool execution with audit logging and progress wiring.
//!
//! Extracted from [`super::prompt_events`] to keep that loop within the file
//! budget. Owns the registry lookup, policy gate, progress-id injection, and
//! success/failure audit records for one tool call. Audit/progress helpers live
//! in [`tool_exec_audit`].

#[path = "tool_exec_audit.rs"]
mod tool_exec_audit;
#[path = "tool_exec_failure.rs"]
mod tool_exec_failure;
#[path = "tool_exec_run.rs"]
mod tool_exec_run;

use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tool::ToolRegistry;

use tool_exec_audit::{progress_input, unknown_tool};

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
    if let Some(blocked) =
        super::super::workspace_coordination::blocked(tool_name, exec_input).await
    {
        return blocked;
    }
    let _guard = progress.map(|(tx, id)| crate::tool::progress::register(id, tool_name, tx));
    let exec_input = progress_input(exec_input, progress.map(|(_, id)| id));
    tool_exec_run::run(tool, exec_input, tool_name, session_id, exec_start).await
}
