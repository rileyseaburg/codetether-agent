//! Shared owner-scoped transcript lookup through native session access.
use crate::provider::Message;
pub(crate) fn agent_tool_transcript_for_parent(
    name: &str,
    parent_session_id: &str,
) -> Option<Vec<Message>> {
    codetether_agent_view::transcript::<super::ApplicationHost>(name, parent_session_id)
}
#[cfg(test)]
#[path = "bridge_transcript_tests.rs"]
mod tests;
