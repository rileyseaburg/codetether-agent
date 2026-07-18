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

mod actions;
pub mod bridge;
mod bus_publish;
pub(crate) mod collaboration_runtime;
pub(crate) mod communication;
mod dispatch;
mod event_loop;
mod execution_state;
mod handlers;
mod helpers;
mod message;
mod message_detach;
mod message_finalize;
mod message_result;
mod params;
#[cfg(test)]
#[path = "params_tests.rs"]
mod params_tests;
pub(crate) mod persistence;
mod policy;
mod policy_constants;
mod policy_free;
mod policy_parse;
mod policy_registry;
mod registry;
pub(crate) mod residency;
mod session_factory;
mod spawn;
#[cfg(test)]
#[path = "spawn_detach_tests.rs"]
mod spawn_detach_tests;
mod spawn_messages;
mod spawn_request;
mod spawn_run;
mod spawn_store;
mod spawn_validation;
mod status;
mod status_liveness;
#[cfg(test)]
mod status_liveness_tests;
mod status_source;
mod status_tool_activity;
mod store;
mod text;
mod thread_lifecycle;
mod tool_impl;
mod tool_schema;

#[cfg(test)]
mod spawn_tests;

pub use tool_impl::AgentTool;
