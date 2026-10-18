//! Application session and executor bindings for native worker swarm policy.
#[cfg(test)]
mod access;
mod entry;
mod session;
pub(super) use entry::execute_swarm_with_policy;
