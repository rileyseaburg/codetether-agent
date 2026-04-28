use crate::a2a::{bridge, proto};
use crate::actor::{ActorEnvelope, ActorId};

/// Converts generated A2A protobuf messages into actor mailbox envelopes.
pub fn proto_to_actor(from: &str, to: &str, message: &proto::Message) -> ActorEnvelope {
    let local = bridge::proto_message_to_local(message);
    local_to_actor(from, to, &local)
}

/// Converts actor envelopes back into generated A2A protobuf messages.
pub fn actor_to_proto(envelope: &ActorEnvelope) -> proto::Message {
    let local = actor_to_local(envelope);
    bridge::local_message_to_proto(&local)
}

pub(crate) fn local_to_actor(
    from: &str,
    to: &str,
    message: &crate::a2a::types::Message,
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

pub(crate) fn actor_to_local(envelope: &ActorEnvelope) -> crate::a2a::types::Message {
    serde_json::from_value(envelope.payload.clone()).unwrap_or_else(|e| {
        tracing::warn!("Failed to deserialize actor payload to a2a message: {e}. Returning empty message.");
        empty_message(envelope)
    })
}

fn empty_message(envelope: &ActorEnvelope) -> crate::a2a::types::Message {
    crate::a2a::types::Message {
        message_id: envelope.id.clone(),
        role: crate::a2a::types::MessageRole::Agent,
        parts: vec![],
        context_id: envelope.correlation_id.clone(),
        task_id: None,
        metadata: Default::default(),
        extensions: vec!["codetether.actor".to_string()],
    }
}
