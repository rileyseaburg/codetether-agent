//! Timestamped actor messages with correlation metadata.

use super::id::ActorId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Typed message delivered to one actor mailbox.
///
/// # Examples
/// ```
/// let envelope = codetether_actor::ActorEnvelope::new("a".into(), "b".into(), "run", serde_json::json!({}));
/// assert_eq!(envelope.kind, "run");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorEnvelope {
    /// Unique envelope identifier.
    pub id: String,
    /// Sending actor.
    pub from: ActorId,
    /// Destination actor.
    pub to: ActorId,
    /// Message kind used for dispatch.
    pub kind: String,
    /// Structured message payload.
    pub payload: serde_json::Value,
    /// Optional request/response correlation.
    pub correlation_id: Option<String>,
    /// UTC envelope creation time.
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
