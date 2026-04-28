use crate::a2a::types::Message;
use crate::actor::ActorEnvelope;

/// Converts local serde A2A messages into actor envelopes.
pub fn message_to_actor(from: &str, to: &str, message: &Message) -> ActorEnvelope {
    super::proto::local_to_actor(from, to, message)
}

/// Converts an actor envelope carrying `a2a.message` into a local A2A message.
pub fn actor_to_message(envelope: &ActorEnvelope) -> Message {
    super::proto::actor_to_local(envelope)
}
