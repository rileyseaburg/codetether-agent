//! Parent-scoped transcript access for TUI agent detail views.

use crate::Host;
use codetether_provider_core::Message;

/// Returns a cloned transcript for an agent owned by `parent_session_id`.
pub fn for_parent<H: Host>(name: &str, parent_session_id: &str) -> Option<Vec<Message>> {
    H::get_for_parent(name, parent_session_id)
        .map(|entry| H::into_messages(entry.session))
        .or_else(|| codetether_peer_agent::observation::transcript(name, parent_session_id))
}
