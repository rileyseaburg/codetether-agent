//! JSON payload conversion and malformed-payload fallback for A2A messages.
use crate::{ActorEnvelope, ActorId};

pub(crate) fn local_to_actor(
    from: &str,
    to: &str,
    message: &codetether_a2a_core::Message,
) -> ActorEnvelope {
    let payload = serde_json::to_value(message).unwrap_or_else(|e| {
        tracing::error!("Failed to serialize a2a message to JSON: {e}");
        serde_json::Value::Null
    });
    ActorEnvelope::new(
        ActorId::from(from),
        ActorId::from(to),
        "a2a.message",
        payload,
    )
}

pub(crate) fn actor_to_local(envelope: &ActorEnvelope) -> codetether_a2a_core::Message {
    serde_json::from_value(envelope.payload.clone()).unwrap_or_else(|e| {
        tracing::warn!(
            "Failed to deserialize actor payload to a2a message: {e}. Returning empty message."
        );
        empty_message(envelope)
    })
}

fn empty_message(envelope: &ActorEnvelope) -> codetether_a2a_core::Message {
    codetether_a2a_core::Message {
        message_id: envelope.id.clone(),
        role: codetether_a2a_core::MessageRole::Agent,
        parts: vec![],
        context_id: envelope.correlation_id.clone(),
        task_id: None,
        metadata: Default::default(),
        extensions: vec!["codetether.actor".to_string()],
    }
}
