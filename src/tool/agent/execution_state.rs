//! Per-agent execution and close state for spawned child sessions.

#[path = "execution_state/guard.rs"]
mod guard;
#[path = "execution_state/lifecycle.rs"]
mod lifecycle;
#[path = "execution_state/registry.rs"]
mod registry;

pub(super) use guard::AgentRunGuard;
pub(super) use lifecycle::{close, is_closed, reopen};
pub(super) use registry::{abort, is_running, register, try_start};

#[cfg(test)]
#[path = "execution_state_tests.rs"]
mod tests;
