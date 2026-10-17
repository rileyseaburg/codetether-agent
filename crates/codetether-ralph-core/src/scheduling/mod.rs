//! Sequential and dependency-stage scheduling over application host callbacks.
//!
//! [run_sequential] selects and handles stories within iteration limits.
//! [run_parallel] preserves stage order and a shared worktree-manager lifetime.
//! Hosts retain providers, command policy, worktree creation and event delivery.
use crate::{RalphState, UserStory};
use anyhow::Result;
mod parallel;
mod parallel_host;
mod sequential;
mod sequential_host;
pub use parallel::run_parallel;
pub use parallel_host::ParallelHost;
pub use sequential::run_sequential;
pub use sequential_host::SequentialHost;
