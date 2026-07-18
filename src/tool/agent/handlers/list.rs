//! Rendering for the legacy list action.

use super::super::store;
use crate::tool::ToolResult;
use serde_json::json;

pub(in crate::tool::agent) fn handle(parent: Option<&str>) -> ToolResult {
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
    if agents.is_empty() {
        return ToolResult::success("No local or LAN agents are available yet.");
    }
    ToolResult::success(serde_json::to_string_pretty(&agents).unwrap_or_default())
}
