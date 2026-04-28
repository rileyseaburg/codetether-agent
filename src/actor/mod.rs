//! Actor-style messaging runtime for addressable agents.

pub mod a2a;
pub mod bus;
pub mod dead_letter;
pub mod envelope;
pub mod id;
pub mod mailbox;
pub mod message;
pub mod receipt;
pub mod runtime;
pub mod state;

#[cfg(test)]
mod tests;

pub use bus::{decode_actor, publish_actor};
pub use envelope::ActorEnvelope;
pub use id::ActorId;
pub use mailbox::{ActorMailbox, MailboxDelivery};
pub use message::ActorMessage;
pub use receipt::ActorAck;
pub use runtime::ActorRuntime;
pub use state::{DeliveryRecord, DeliveryState};
