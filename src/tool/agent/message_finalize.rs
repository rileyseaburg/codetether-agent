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
    name: String,
    response: String,
    thinking: String,
    tools: Vec<Value>,
    error: Option<String>,
    updated_session: Option<Session>,
) -> ToolResult {
    if let Some(updated) = updated_session {
        store::update_session(&name, updated.clone());
        super::event_loop::live_trace::clear(&name);
        if let Err(e) = updated.save().await {
            tracing::warn!(agent = %name, error = %e, "Failed to save agent session after message");
        }
    }
    let error = message_result::effective_error(&response, error);
    let success = error.is_none();
    let summary = lifecycle::terminal_summary(tools.len(), error.as_deref());
    bus_publish::announce_result(&name, &response, error.as_deref());
    bus_publish::announce_done(&name, success, summary);
    message_result::build_message_result(name, response, thinking, tools, error)
}
