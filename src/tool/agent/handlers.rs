//! Simple action handlers (list, kill).

use super::store;
use crate::tool::ToolResult;
use serde_json::json;

/// Lists sub-agents owned by the calling session as a JSON array.
///
/// # Examples
///
/// ```ignore
/// let result = handle_list();
/// ```
pub(super) fn handle_list(parent: Option<&str>) -> ToolResult {
    let mut agents = store::list_for_parent(parent)
        .into_iter()
        .map(|(name, instructions, messages)| {
            json!({ "name": name, "instructions": instructions, "messages": messages })
        })
        .collect::<Vec<_>>();
    agents.extend(super::message::remote::list());
    if agents.is_empty() {
        return ToolResult::success("No local or LAN agents are available yet.");
    }
    ToolResult::success(serde_json::to_string_pretty(&agents).unwrap_or_default())
}

/// Removes a spawned sub-agent from the in-memory store.
///
/// # Examples
///
/// ```ignore
/// let result = handle_kill("reviewer");
/// ```
pub(super) fn handle_kill(name: &str, parent: Option<&str>) -> ToolResult {
    match store::get_for_parent(name, parent) {
        Some(_) => {
            let aborted = super::execution_state::abort(name);
            tracing::info!(agent = %name, aborted, "Sub-agent killed");
            super::bus_publish::announce_done(name, false, "killed by user");
            store::remove(name);
            super::event_loop::live_trace::clear(name);
            ToolResult::success(format!("Terminated and removed @{name}"))
        }
        None => ToolResult::error(format!("Agent @{name} not found")),
    }
}
