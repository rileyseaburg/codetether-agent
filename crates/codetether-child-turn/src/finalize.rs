//! Post-message-loop finalization: persists the updated sub-agent session,
//! announces completion on the bus, and builds the tool result.

use crate::FinalizationHost;
use codetether_agent_messages as message_result;
use codetether_agent_status as lifecycle;
use codetether_collaboration_state::{parent_activity, thread_status};
use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Persist updated state, notify the owner, publish terminal status and render output.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// * `response` — Collected response text.
/// * `thinking` — Collected reasoning text.
/// * `tools` — Tool-call previews.
/// * `error` — Existing execution error.
/// * `updated_session` — Session returned by the child task, if any.
/// # Returns
/// The structured result with runtime/deliverable failure precedence preserved.
/// # Examples
/// Use an application [FinalizationHost] implementation for native effects.
pub async fn finalize<H: FinalizationHost>(
    agent_id: String,
    response: String,
    thinking: String,
    tools: Vec<Value>,
    error: Option<String>,
    updated_session: Option<H::Session>,
) -> ToolResult {
    H::clear_steering(&agent_id);
    if let Some(updated) = updated_session {
        H::update_session(&agent_id, updated.clone());
        codetether_agent_trace::clear(&agent_id);
        if let Err(e) = H::save(&updated).await {
            tracing::warn!(target: "codetether_agent::tool::agent::message_finalize", agent_id, error = %e, "Failed to save agent session after message");
        }
    }
    let error = message_result::effective_error(&response, error);
    thread_status::turn(&agent_id, &response, error.as_deref());
    let success = error.is_none();
    let summary = lifecycle::terminal_summary(tools.len(), error.as_deref());
    H::announce_result(&agent_id, &response, error.as_deref());
    if let Some(owner) = H::lookup(&agent_id).and_then(|entry| entry.owner_session_id) {
        parent_activity::mailbox(&owner);
    }
    H::announce_done(&agent_id, success, summary);
    let name = H::lookup(&agent_id)
        .map(|entry| entry.name)
        .unwrap_or_default();
    message_result::build_message_result(agent_id, name, response, thinking, tools, error)
}
