//! Shared mux argument schema with native CLI integration tests.
pub use codetether_mux_cli::{MuxArgs, MuxCommand, MuxStartOptions};

#[cfg(test)]
#[path = "mux_args_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "mux_args_kill_tests.rs"]
mod kill_tests;

#[cfg(test)]
#[path = "mux_args_help_tests.rs"]
mod help_tests;

#[cfg(test)]
#[path = "mux_args_resume_tests.rs"]
mod resume_tests;

#[cfg(test)]
#[path = "mux_args_worktree_tests.rs"]
mod worktree_tests;
