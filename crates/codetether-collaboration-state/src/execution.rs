//! Per-agent execution and close state for spawned child sessions.

#[path = "execution/guard.rs"]
mod guard;
#[path = "execution/lifecycle.rs"]
mod lifecycle;
#[path = "execution/registry.rs"]
mod registry;

pub use guard::AgentRunGuard;
pub use lifecycle::{close, is_closed, reopen};
pub use registry::{abort, is_running, register, try_start};

#[cfg(test)]
#[path = "execution/tests.rs"]
mod tests;
