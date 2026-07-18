//! Post-message-loop finalization: persists the updated sub-agent session,
//! announces completion on the bus, and builds the tool result.

use super::bus_publish;
use super::bus_publish::lifecycle;
use super::message_result;
use super::store;
use crate::session::Session;
use crate::tool::ToolResult;
use serde_json::Value;

/// Persist any updated session, announce done on the bus, and render the result.
pub(super) async fn finalize(
    agent_id: String,
    response: String,
    thinking: String,
    tools: Vec<Value>,
    error: Option<String>,
    updated_session: Option<Session>,
) -> ToolResult {
    crate::session::helper::steering::clear(&agent_id);
    if let Some(updated) = updated_session {
        store::update_session(&agent_id, updated.clone());
        super::event_loop::live_trace::clear(&agent_id);
        if let Err(e) = updated.save().await {
            tracing::warn!(agent_id, error = %e, "Failed to save agent session after message");
        }
    }
    let error = message_result::effective_error(&response, error);
    super::collaboration_runtime::thread_status::turn(&agent_id, &response, error.as_deref());
    let success = error.is_none();
    let summary = lifecycle::terminal_summary(tools.len(), error.as_deref());
    bus_publish::announce_result(&agent_id, &response, error.as_deref());
    if let Some(owner) = store::get(&agent_id).and_then(|entry| entry.owner_session_id) {
        super::collaboration_runtime::parent_activity::mailbox(&owner);
    }
    bus_publish::announce_done(&agent_id, success, summary);
    let name = store::get(&agent_id)
        .map(|entry| entry.name)
        .unwrap_or_default();
    message_result::build_message_result(agent_id, name, response, thinking, tools, error)
}
