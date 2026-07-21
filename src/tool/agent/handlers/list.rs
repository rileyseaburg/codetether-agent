//! Rendering for the legacy list action.

use super::super::store;
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

pub(in crate::tool::agent) async fn handle(parent: Option<&str>) -> Result<ToolResult> {
    let mut agents = store::entries_for_parent(parent)
        .into_iter()
        .map(|entry| {
            json!({
                "agent_id": entry.id(), "name": entry.name,
                "instructions": entry.instructions, "messages": entry.session.messages.len()
            })
        })
        .collect::<Vec<_>>();
    agents.extend(super::super::message::remote::list());
    agents.extend(crate::mux::control::agent_sessions().await?);
    if agents.is_empty() {
        return Ok(ToolResult::success(
            "No local, LAN, or mux agents are available yet.",
        ));
    }
    Ok(ToolResult::success(
        serde_json::to_string_pretty(&agents).unwrap_or_default(),
    ))
}
