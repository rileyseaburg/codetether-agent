//! Render status rows after reading owned lifecycle and tool activity.
use serde_json::Value;

use super::status_source::latest_states;
use crate::Host;
use crate::row::row;
use codetether_tool_core::ToolResult;

/// Build the existing JSON status report for the owner's live child snapshots.
/// # Arguments
/// * `parent` — Optional owner identity passed to application snapshot lookups.
/// # Returns
/// A successful tool result with status rows or the existing empty-state message.
/// # Examples
/// Call with an application [Host] implementation to inspect its live children.
pub fn handle_status<H: Host>(parent: Option<&str>) -> ToolResult {
    let agents = H::agents(parent);
    if agents.is_empty() {
        return ToolResult::success("No sub-agents spawned. Use action \"spawn\".");
    }
    let states = latest_states(&H::task_to_agent(parent));
    let rows: Vec<Value> = agents
        .iter()
        .map(|entry| {
            row(
                &entry.name,
                &entry.id,
                entry.messages,
                states.get(&entry.id),
            )
        })
        .collect();
    ToolResult::success(serde_json::to_string_pretty(&rows).unwrap_or_default())
}
