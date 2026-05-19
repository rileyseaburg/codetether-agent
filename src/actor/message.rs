use super::{envelope::ActorEnvelope, id::ActorId};

/// Builder for strongly named actor commands/events.
#[derive(Debug, Clone)]
pub struct ActorMessage {
    from: ActorId,
    to: ActorId,
    kind: String,
    payload: serde_json::Value,
}

impl ActorMessage {
    /// Creates a new actor message builder.
    pub fn new(
        from: impl Into<ActorId>,
        to: impl Into<ActorId>,
        kind: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            kind: kind.into(),
            payload,
        }
    }

    /// Converts the builder into a deliverable envelope.
    pub fn into_envelope(self) -> ActorEnvelope {
        ActorEnvelope::new(self.from, self.to, self.kind, self.payload)
    }
}
