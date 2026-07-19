//! Read-only, parent-scoped queries over observed remote turns.

use super::store::{TURNS, key, owned};
use super::types::RemoteSnapshot;
use crate::provider::Message;

/// Lists remote peers observed by one parent, or every parent when absent.
pub(in crate::tool::agent) fn snapshots(owner_session_id: Option<&str>) -> Vec<RemoteSnapshot> {
    TURNS
        .iter()
        .filter(|entry| owned(entry.value(), owner_session_id))
        .map(|entry| RemoteSnapshot::from(entry.value()))
        .collect()
}

/// Returns the retained request/result transcript for one owned peer.
pub(in crate::tool::agent) fn transcript(
    name: &str,
    owner_session_id: &str,
) -> Option<Vec<Message>> {
    TURNS
        .get(&key(name, Some(owner_session_id)))
        .map(|turn| turn.messages.clone())
}
