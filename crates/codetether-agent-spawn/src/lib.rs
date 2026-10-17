//! Managed agent spawn orchestration and result contracts.
//!
//! Implement [Construction] and [Host] for existing session and execution
//! operations. [handle_spawn] validates input, holds identity/capacity guards,
//! persists durable children and dispatches their first turn. [ephemeral_result]
//! maps existing one-shot execution outcomes without changing their semantics.
mod construction;
mod dispatch;
mod durable;
mod ephemeral_result;
mod finish;
mod host;
mod messages;
mod prepare;
pub use construction::Construction;
pub use dispatch::handle_spawn;
pub use ephemeral_result::build as ephemeral_result;
pub use host::Host;

mod registration;
mod registration_host;
pub use registration::persist_spawned_agent;
pub use registration_host::Registration;
