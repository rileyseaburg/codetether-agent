//! Observable Codex-compatible child-thread lifecycle status.

#[path = "thread_status/registry.rs"]
mod registry;
#[path = "thread_status/settle.rs"]
mod settle;
#[path = "thread_status/state.rs"]
mod state;
#[path = "thread_status/transitions.rs"]
mod transitions;

pub(crate) use registry::{get, initialize, remove, set, subscribe};
pub(crate) use settle::turn;
pub(crate) use state::ThreadStatus;
pub(crate) use transitions::{interrupted, restored, running, shutdown, track_error};

#[cfg(test)]
#[path = "thread_status/tests.rs"]
mod tests;
