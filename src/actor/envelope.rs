use super::id::ActorId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Typed message delivered to one actor mailbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorEnvelope {
    pub id: String,
    pub from: ActorId,
    pub to: ActorId,
    pub kind: String,
    pub payload: serde_json::Value,
    pub correlation_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ActorEnvelope {
    /// Creates a typed envelope for mailbox delivery.
    pub fn new(
        from: ActorId,
        to: ActorId,
        kind: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from,
            to,
            kind: kind.into(),
            payload,
            correlation_id: None,
            timestamp: Utc::now(),
        }
    }

    /// Adds a correlation id for request/response flows.
    pub fn correlated(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }
}
