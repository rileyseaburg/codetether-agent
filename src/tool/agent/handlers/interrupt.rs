//! Active-turn interruption without child removal.

use super::super::store;
use crate::tool::ToolResult;
use anyhow::Result;

#[path = "interrupt/marker.rs"]
mod marker;
#[path = "interrupt/wait.rs"]
mod wait;

/// Interrupt an active child turn and optionally persist its history marker.
pub(in crate::tool::agent) async fn handle(
    target: &str,
    parent: Option<&str>,
    marker_enabled: bool,
) -> Result<ToolResult> {
    let Some((agent_id, entry)) = store::scope::resolve_for_parent(target, parent) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    if super::super::execution_state::abort(&agent_id) {
        super::super::collaboration_runtime::thread_status::interrupted(&agent_id);
        tracing::info!(agent_id, agent = %entry.name, "Sub-agent turn interrupted");
        super::super::bus_publish::announce_done(&agent_id, false, "interrupted");
        if !wait::until_idle(&agent_id).await {
            tracing::warn!(agent_id, agent = %entry.name, "Sub-agent interrupt did not settle");
            return Ok(ToolResult::error(format!(
                "Interrupt requested for @{}, but its turn did not stop within 5 seconds",
                entry.name
            )));
        }
        if marker_enabled {
            marker::persist(&agent_id).await?;
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

#[cfg(test)]
#[path = "interrupt/idle_tests.rs"]
mod idle_tests;
#[cfg(test)]
#[path = "interrupt/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "interrupt/tests.rs"]
mod tests;
