//! Mux-owned structured CodeTether agent task lifecycle.
//!
//! [AgentTaskRegistry] admits one running turn per mux session, starts an agent
//! child process, retains bounded output and serves session-scoped replay reads.
//! Construct a registry with [AgentTaskRegistry::new], then use its start, read
//! and cancellation methods. Process groups keep child cancellation scoped.

mod buffer;
mod cancel;
mod entry;
mod entry_read;
mod executable;
mod reader;
mod registry;
mod registry_cancel;
mod registry_read;
mod registry_start;
mod spawn;
mod spawn_group;
mod store;
mod validation;
mod waiter;

#[cfg(test)]
mod isolation_tests;
#[cfg(test)]
mod tests;

pub use registry::AgentTaskRegistry;

mod child_io;
mod store_access;
