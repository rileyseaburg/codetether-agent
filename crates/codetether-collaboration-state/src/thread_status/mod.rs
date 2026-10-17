//! Observable Codex-compatible child-thread lifecycle status.

#[path = "registry.rs"]
mod registry;
#[path = "settle.rs"]
mod settle;
#[path = "state.rs"]
mod state;
#[path = "transitions.rs"]
mod transitions;

pub use registry::{get, initialize, remove, set, subscribe};
pub use settle::turn;
pub use state::ThreadStatus;
pub use transitions::{interrupted, restored, running, shutdown, track_error};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
