//! Parent-scoped transcript access for TUI agent detail views.

use super::store;
use crate::provider::Message;

/// Returns a cloned transcript for an agent owned by `parent_session_id`.
pub(crate) fn agent_tool_transcript_for_parent(
    name: &str,
    parent_session_id: &str,
) -> Option<Vec<Message>> {
    store::get_for_parent(name, Some(parent_session_id))
        .map(|entry| entry.session.messages)
        .or_else(|| super::super::message::remote::observation::transcript(name, parent_session_id))
}

#[cfg(test)]
#[path = "bridge_transcript_tests.rs"]
mod tests;
