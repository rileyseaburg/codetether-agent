//! Read-only, parent-scoped queries over observed remote turns.

use super::store::{TURNS, key, owned};
use super::types::RemoteSnapshot;
use codetether_agent_trace::LiveTraceSnapshot;
use codetether_provider_core::Message;

/// Lists remote peers observed by one parent, or every parent when absent.
/// # Arguments
/// `owner_session_id` filters observed turns; none lists all owners.
/// # Returns
/// Owned peer summaries for the requested scope.
/// # Examples
/// Start a turn with [crate::observation::begin] before reading its evidence.
pub fn snapshots(owner_session_id: Option<&str>) -> Vec<RemoteSnapshot> {
    TURNS
        .iter()
        .filter(|entry| owned(entry.value(), owner_session_id))
        .map(|entry| RemoteSnapshot::from(entry.value()))
        .collect()
}

/// Returns the retained request/result transcript for one owned peer.
/// # Arguments
/// `name` and `owner_session_id` select one exact conversation.
/// # Returns
/// The retained provider-message transcript, or none when absent.
/// # Examples
/// Start a turn with [crate::observation::begin] before reading its evidence.
pub fn transcript(name: &str, owner_session_id: &str) -> Option<Vec<Message>> {
    TURNS
        .get(&key(name, Some(owner_session_id)))
        .map(|turn| turn.messages.clone())
}

/// Returns the latest structured activity while an owned peer is working.
/// # Arguments
/// `name` and `owner_session_id` select one exact conversation.
/// # Returns
/// The projected live trace while processing, otherwise none.
/// # Examples
/// Start a turn with [crate::observation::begin] before reading its evidence.
pub fn live_trace(name: &str, owner_session_id: &str) -> Option<LiveTraceSnapshot> {
    let turn = TURNS.get(&key(name, Some(owner_session_id)))?;
    super::projection::trace(&turn)
}
