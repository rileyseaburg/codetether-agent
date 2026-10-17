//! In-process actor lifecycle and coordination-message contracts.
//!
//! Implement [Actor] and [Handler] in the execution layer; exchange [SwarmMessage]
//! values and expose lifecycle state through [ActorStatus].
use anyhow::Result;
use async_trait::async_trait;
mod actor;
pub use actor::Actor;
mod handler;
pub use handler::Handler;
mod message;
pub use message::SwarmMessage;
mod status;
pub use status::ActorStatus;

mod subagent_message;
pub use subagent_message::SubAgentMessage;
mod orchestrator_message;
pub use orchestrator_message::OrchestratorMessage;
