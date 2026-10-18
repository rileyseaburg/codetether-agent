//! Actor processing acknowledgments.

use serde::{Deserialize, Serialize};

/// Acknowledgement returned by an actor after handling a message.
///
/// # Examples
/// ```
/// let ack = codetether_actor::ActorAck::new("message-1", "worker");
/// assert_eq!(ack.actor_id, "worker");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAck {
    /// Acknowledged message identifier.
    pub message_id: String,
    /// Actor that acknowledged the message.
    pub actor_id: String,
}

impl ActorAck {
    /// Creates an acknowledgement for a handled message.
    pub fn new(message_id: impl Into<String>, actor_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            actor_id: actor_id.into(),
        }
    }
}
