//! Permanent removal of one canonical child thread.

use crate::ControlHost;
use codetether_collaboration_state::{execution, thread_status};
use codetether_tool_core::ToolResult;

/// Permanently remove a live child resolved within its owner scope.
/// # Arguments
/// * `target` — Canonical ID, path or compatible child name.
/// * `parent` — Optional owner scope.
/// # Returns
/// Existing not-found or removal result text.
/// # Errors
/// Propagates mailbox and persistence errors before live removal.
/// # Examples
/// Use an application [ControlHost] implementation.
pub async fn kill<H: ControlHost>(
    target: &str,
    parent: Option<&str>,
) -> anyhow::Result<ToolResult> {
    let Some((agent_id, entry)) = H::resolve(target, parent) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let aborted = execution::abort(&agent_id);
    thread_status::remove(&agent_id);
    H::clear_mailbox(&agent_id).await?;
    H::remove(&entry).await?;
    tracing::info!(target: "codetether_agent::tool::agent::handlers::kill", agent_id, agent = %entry.name, aborted, "Sub-agent killed");
    H::announce_done(&agent_id, false, "killed by user");
    H::remove_runtime(&agent_id);
    Ok(ToolResult::success(format!(
        "Terminated and removed @{} ({agent_id})",
        entry.name
    )))
}
