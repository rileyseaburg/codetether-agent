use super::{ActorEnvelope, ActorId};

/// Builds the dead-letter topic for an actor.
pub fn topic(actor: &ActorId) -> String {
    format!("actor.{}.dead", actor.0)
}

/// Annotates an envelope as dead-lettered for observability.
pub fn mark(mut envelope: ActorEnvelope, reason: impl Into<String>) -> ActorEnvelope {
    let reason = reason.into();
    envelope.kind = format!("{}.dead", envelope.kind);
    envelope.payload = serde_json::json!({
        "reason": reason,
        "original_payload": envelope.payload,
    });
    envelope
}
