//! Mux-owned structured CodeTether agent task lifecycle.

mod buffer;
mod cancel;
mod entry;
mod entry_read;
mod reader;
mod registry;
mod registry_start;
mod spawn;
mod spawn_group;
mod store;
mod validation;
mod waiter;

#[cfg(test)]
mod tests;

pub(in crate::mux) use registry::AgentTaskRegistry;
