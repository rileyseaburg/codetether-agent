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
mod event_loop;
mod execution_state;
#[cfg(test)]
mod execution_state_tests;
mod handlers;
mod helpers;
mod message;
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
mod spawn_request;
mod spawn_store;
mod spawn_validation;
mod store;
mod text;
mod tool_impl;
mod tool_schema;

pub use tool_impl::AgentTool;
