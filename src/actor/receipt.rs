use serde::{Deserialize, Serialize};

/// Acknowledgement returned by an actor after handling a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAck {
    pub message_id: String,
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
