//! Direct journal entrypoints shared with application integration tests.
use crate::Host;
use anyhow::Result;
use codetether_goal_core::TaskLog;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Dispatch raw legacy arguments against an already resolved journal.
///
/// # Arguments
/// * `log` — Existing session event journal.
/// * `params` — JSON arguments for the requested action.
/// # Returns
/// The action's tool result.
/// # Errors
/// Propagates decoding, validation, journal and verifier errors.
/// # Examples
/// Use [SessionTaskTool](crate::SessionTaskTool) to also resolve a session path.
pub async fn dispatch<H: Host>(log: &TaskLog, params: Value) -> Result<ToolResult> {
    crate::dispatch::run::<H>(log, serde_json::from_value(params)?).await
}
/// Render work items from a journal without changing its goal.
/// # Arguments
/// * `log` — Existing session event journal.
/// # Returns
/// A task-only listing.
/// # Errors
/// Propagates journal read errors.
/// # Examples
/// ```no_run
/// # async fn example()->anyhow::Result<()> {
/// let log=codetether_goal_core::TaskLog::at("tasks.jsonl");
/// let result=codetether_session_task::list_tasks(&log).await?;
/// assert!(result.success);
/// # Ok(()) }
/// ```
pub async fn list_tasks(log: &TaskLog) -> Result<ToolResult> {
    crate::handlers::list(log).await
}
