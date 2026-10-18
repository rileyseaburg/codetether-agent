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

mod ephemeral;
mod ephemeral_host;
mod ephemeral_run;
mod ephemeral_setup;
pub use ephemeral::run as run_ephemeral;
pub use ephemeral_host::{EphemeralHost, EphemeralPolicy, EphemeralResources};
pub use ephemeral_run::EphemeralRun;
pub use ephemeral_setup::task::workspace_policy;

mod arguments;
pub use arguments::{SpawnArgs, parse_spawn_args};
#[cfg(test)]
mod arguments_tests;

mod application_binding;

/// Dependencies used by application-local spawn declarations.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use async_trait::async_trait;
    pub use codetether_agent_input::{Params, SpawnRequest};
    pub use codetether_child_workspace::Handoff;
    pub use codetether_collaboration_state::residency::Slot;
    pub use codetether_spawn_policy::Reservation;
    pub use codetether_tool_core::ToolResult;
}
