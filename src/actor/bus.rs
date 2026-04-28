use super::{ActorEnvelope, ActorId};
use crate::bus::{BusEnvelope, BusHandle, BusMessage};

/// Publishes actor envelopes over the existing topic bus.
pub fn publish_actor(handle: &BusHandle, envelope: &ActorEnvelope) -> usize {
    handle.send(
        envelope.to.inbox_topic(),
        BusMessage::SharedResult {
            key: format!("actor/{}", envelope.id),
            value: serde_json::to_value(envelope).unwrap_or(serde_json::Value::Null),
            tags: vec!["actor_message".to_string(), envelope.kind.clone()],
        },
    )
}

/// Decodes one bus envelope into an actor envelope when it carries one.
pub fn decode_actor(envelope: &BusEnvelope) -> Option<ActorEnvelope> {
    match &envelope.message {
        BusMessage::SharedResult { value, tags, .. }
            if tags.iter().any(|tag| tag == "actor_message") =>
        {
            serde_json::from_value(value.clone()).ok()
        }
        BusMessage::AgentMessage { from, to, parts } => {
            let payload = serde_json::json!({"parts": parts});
            Some(ActorEnvelope::new(
                ActorId::from(from.clone()),
                ActorId::from(to.clone()),
                "a2a.parts",
                payload,
            ))
        }
        _ => None,
    }
}
