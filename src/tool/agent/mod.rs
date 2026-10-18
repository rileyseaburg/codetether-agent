//! Agent tool wiring.
//!
//! This module assembles the focused submodules that implement spawning,
//! messaging, listing, and killing sub-agents while keeping the public surface
//! limited to `AgentTool`.
//!
//! # Examples
//!
//! ```ignore
//! let tool = AgentTool::new();
//! assert_eq!(tool.id(), "agent");
//! ```

pub mod bridge;
mod bus_publish;
pub(crate) mod collaboration_runtime;
pub(crate) mod communication;
#[cfg(test)]
mod dispatch;
mod event_loop;
use codetether_collaboration_state::execution as execution_state;
mod handlers;
mod helpers;
mod message;
mod message_detach;
mod message_finalize;
use codetether_agent_input as params;
pub(crate) mod persistence;
mod registry;
pub(crate) mod residency;
mod session_factory;
mod spawn;
use codetether_agent_input as spawn_request;
mod spawn_run;
mod spawn_store;
mod spawn_validation;
mod status;
mod store;
use codetether_child_events as text;
mod thread_lifecycle;
mod tool_impl;
mod turn_host;

#[cfg(test)]
mod spawn_tests;

pub use tool_impl::AgentTool;
