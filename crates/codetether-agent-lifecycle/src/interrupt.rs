//! Active-turn interruption without child removal.

use crate::MarkerHost;
use anyhow::Result;
use codetether_collaboration_state::{execution, thread_status};
use codetether_tool_core::ToolResult;

use super::{marker, wait};

/// Interrupt an active child turn and optionally persist its history marker.
/// # Arguments
/// * `target` — Child identity resolved in the owner scope.
/// * `parent` — Optional owner scope.
/// * `marker_enabled` — Whether to persist the interruption developer message.
/// # Returns
/// Existing not-found, idle, interrupted or timeout result text.
/// # Errors
/// Propagates marker persistence errors after the active turn settles.
/// # Examples
/// Use an application [MarkerHost] implementation.
pub async fn interrupt<H: MarkerHost>(
    target: &str,
    parent: Option<&str>,
    marker_enabled: bool,
) -> Result<ToolResult> {
    let Some((agent_id, entry)) = H::resolve(target, parent) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    if execution::abort(&agent_id) {
        thread_status::interrupted(&agent_id);
        tracing::info!(target: "codetether_agent::tool::agent::handlers::interrupt", agent_id, agent = %entry.name, "Sub-agent turn interrupted");
        H::announce_done(&agent_id, false, "interrupted");
        if !wait::until_idle(&agent_id).await {
            tracing::warn!(target: "codetether_agent::tool::agent::handlers::interrupt", agent_id, agent = %entry.name, "Sub-agent interrupt did not settle");
            return Ok(ToolResult::error(format!(
                "Interrupt requested for @{}, but its turn did not stop within 5 seconds",
                entry.name
            )));
        }
        if marker_enabled {
            marker::persist::<H>(&agent_id).await?;
        }
        Ok(ToolResult::success(format!(
            "Interrupted @{}; its session is preserved",
            entry.name
        )))
    } else {
        Ok(ToolResult::success(format!(
            "Agent @{} is already idle",
            entry.name
        )))
    }
}
