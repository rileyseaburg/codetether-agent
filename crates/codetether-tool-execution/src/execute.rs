//! Resolve, authorize and execute a tool with scoped progress reporting.
use crate::Host;
use codetether_session_events::SessionEvent;
use codetether_tool_reporting::{ToolTuple, progress_input, unknown_tool};
use tokio::sync::mpsc;

/// Execute one tool and return its `(output, success, metadata)` tuple.
///
/// # Arguments
/// `host` resolves tools and enforces authorization; `tool_name` and `exec_input`
/// identify the call; `session_id` correlates audit entries; `exec_start` measures
/// duration; optional `progress` supplies an event channel and invocation ID.
///
/// # Returns
/// A blocking result, tool output, or a reported execution error. Tool errors
/// retain their full cause chain in failure reporting.
///
/// # Panics
/// Progress registration panics if its shared registry mutex was poisoned.
///
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_tool_execution::Host) {
/// let result = codetether_tool_execution::execute::<()>(
///     host, "read", &serde_json::json!({"path": "README.md"}), "session-1",
///     std::time::Instant::now(), None
/// ).await;
/// let (_output, _success, _metadata) = result;
/// # }
/// ```
pub async fn execute<S: Send + 'static>(
    host: &impl Host,
    tool_name: &str,
    exec_input: &serde_json::Value,
    session_id: &str,
    exec_start: std::time::Instant,
    progress: Option<(&mpsc::Sender<SessionEvent<S>>, &str)>,
) -> ToolTuple {
    let Some(tool) = host.resolve(tool_name) else {
        return unknown_tool(tool_name, session_id).await;
    };
    if let Some(blocked) = host.policy(tool_name, exec_input).await {
        return blocked;
    }
    if let Some(blocked) = host.coordinate(tool_name, exec_input).await {
        return blocked;
    }
    let _guard =
        progress.map(|(tx, id)| codetether_tool_reporting::register_progress(id, tool_name, tx));
    let exec_input = progress_input(exec_input, progress.map(|(_, id)| id));
    crate::run::run(tool, exec_input, tool_name, session_id, exec_start).await
}
