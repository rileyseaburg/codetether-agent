//! State models for one observed remote-agent conversation.

use crate::provider::Message;

#[derive(Clone)]
pub(super) struct RemoteTurn {
    pub name: String,
    pub owner_session_id: Option<String>,
    pub turn_id: String,
    pub messages: Vec<Message>,
    pub is_processing: bool,
    pub failed: bool,
}

/// Read-only remote-peer state used by the TUI bridge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::tool::agent) struct RemoteSnapshot {
    /// Discovered peer name.
    pub name: String,
    /// Retained user and assistant messages.
    pub message_count: usize,
    /// Whether a request is currently in flight.
    pub is_processing: bool,
    /// Whether the most recent request failed.
    pub failed: bool,
}

/// Marks an observed remote turn failed if its future is cancelled.
pub(in crate::tool::agent) struct RemoteTurnGuard {
    pub(super) name: String,
    pub(super) owner_session_id: Option<String>,
    pub(super) turn_id: String,
    pub(super) settled: bool,
}

impl From<&RemoteTurn> for RemoteSnapshot {
    fn from(turn: &RemoteTurn) -> Self {
        Self {
            name: turn.name.clone(),
            message_count: turn.messages.len(),
            is_processing: turn.is_processing,
            failed: turn.failed,
        }
    }
}
