//! Rendering for the legacy list action.

use crate::Host;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::json;

/// Render the existing local-child and discovered-peer agent-tool listing.
/// # Arguments
/// * `parent` — Optional owner scope for local children.
/// # Returns
/// The unchanged JSON roster or empty-state text in a successful tool result.
/// # Errors
/// This function does not currently produce an error.
/// # Examples
/// Use a [Host] that also supplies local-name collisions to the peer roster.
pub fn legacy_list<H: Host + codetether_peer_agent::LocalNames>(
    parent: Option<&str>,
) -> Result<ToolResult> {
    let mut agents = H::entries(parent)
        .into_iter()
        .map(|entry| {
            json!({
                "agent_id": entry.id(), "name": entry.name,
                "instructions": entry.instructions, "messages": H::message_count(&entry.session)
            })
        })
        .collect::<Vec<_>>();
    agents.extend(codetether_peer_agent::list::<H>());
    if agents.is_empty() {
        return Ok(ToolResult::success(
            "No local or LAN agents are available yet.",
        ));
    }
    Ok(ToolResult::success(
        serde_json::to_string_pretty(&agents).unwrap_or_default(),
    ))
}
