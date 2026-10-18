//! Protobuf A2A message conversion through the shared wire bridge.

use crate::{ActorEnvelope, ActorId};
use codetether_a2a_grpc::{bridge, proto};

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

pub(crate) use super::local_codec::{actor_to_local, local_to_actor};
