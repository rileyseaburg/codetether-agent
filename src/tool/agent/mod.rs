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
mod bus_publish;
mod event_loop;
mod execution_state;
#[cfg(test)]
mod execution_state_tests;
mod handlers;
mod helpers;
mod message;
mod message_finalize;
mod message_result;
mod params;
mod policy;
mod policy_constants;
mod policy_free;
mod policy_parse;
mod policy_registry;
mod registry;
mod session_factory;
mod spawn;
mod spawn_messages;
mod spawn_request;
mod spawn_store;
mod spawn_validation;
#[cfg(test)]
mod spawn_validation_tests;
mod store;
mod text;
mod tool_impl;
mod tool_schema;

#[cfg(test)]
mod spawn_tests;

pub use tool_impl::AgentTool;
