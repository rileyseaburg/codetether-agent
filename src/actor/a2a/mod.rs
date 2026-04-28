//! A2A bridge for actor mailbox messages.

pub mod local;
pub mod proto;

pub use local::{actor_to_message, message_to_actor};
pub use proto::{actor_to_proto, proto_to_actor};
