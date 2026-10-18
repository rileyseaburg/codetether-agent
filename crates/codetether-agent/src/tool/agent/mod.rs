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
codetether_agent_status::declare_application_status!(
    bus_publish, status; crate::tool::agent::store
);
pub(crate) mod collaboration_runtime;
pub(crate) mod communication;
#[cfg(test)]
mod dispatch;
mod event_loop;
mod handlers;
mod helpers;
mod message;
codetether_child_turn::declare_turn_entrypoints!(
    crate::tool::agent::turn_host::ApplicationHost,
    crate::session::Session,
    crate::session::SessionEvent,
);
pub(crate) mod persistence;
mod registry;
pub(crate) mod residency;
mod session_factory;
mod spawn;
use codetether_agent_input as spawn_request;
mod spawn_store;
mod spawn_validation;
mod store;
mod thread_lifecycle;
mod tool_impl;
mod turn_host;

#[cfg(test)]
mod spawn_tests;

pub use tool_impl::AgentTool;

#[cfg(test)]
include!("mod_test_imports.rs");
