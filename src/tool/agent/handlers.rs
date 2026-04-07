//! Simple action handlers (list, kill).

use super::store;
use crate::tool::ToolResult;
use serde_json::{Value, json};

pub(super) fn handle_list() -> ToolResult {
    let agents = store::list();
    if agents.is_empty() {
        return ToolResult::success("No sub-agents spawned. Use action \"spawn\".");
    }
    let list: Vec<Value> = agents
        .into_iter()
        .map(|(name, instructions, msgs)| {
            json!({ "name": name, "instructions": instructions, "messages": msgs })
        })
        .collect();
    ToolResult::success(serde_json::to_string_pretty(&list).unwrap_or_default())
}

pub(super) fn handle_kill(name: &str) -> ToolResult {
    match store::remove(name) {
        Some(_) => {
            tracing::info!(agent = %name, "Sub-agent killed");
            ToolResult::success(format!("Removed @{name}"))
        }
        None => ToolResult::error(format!("Agent @{name} not found")),
    }
}
