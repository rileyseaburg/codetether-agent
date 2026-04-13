//! Simple action handlers (list, kill).

use super::store;
use crate::tool::ToolResult;
use serde_json::{Value, json};

/// Lists all currently spawned sub-agents as a JSON array.
///
/// # Examples
///
/// ```ignore
/// let result = handle_list();
/// ```
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

/// Removes a spawned sub-agent from the in-memory store.
///
/// # Examples
///
/// ```ignore
/// let result = handle_kill("reviewer");
/// ```
pub(super) fn handle_kill(name: &str) -> ToolResult {
    match store::remove(name) {
        Some(_) => {
            tracing::info!(agent = %name, "Sub-agent killed");
            ToolResult::success(format!("Removed @{name}"))
        }
        None => ToolResult::error(format!("Agent @{name} not found")),
    }
}
