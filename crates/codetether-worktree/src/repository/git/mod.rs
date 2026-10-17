//! Repository preparation through Git subprocesses.
//!
//! [git_clone_repo] prepares a fresh clone; [refresh_existing_clone] updates
//! an existing one after stashing dirty state. [ensure_metadata_checked_out]
//! applies a requested task branch. [run_git_command_at] is their shared
//! command runner and returns trimmed standard output.
use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;
mod branch;
mod checkout;
mod clone;
mod command;
mod current_branch;
mod refresh;
mod stash;
pub use branch::ensure_metadata_checked_out;
use checkout::ensure_checked_out;
pub use clone::git_clone_repo;
pub use command::run_git_command_at;
use current_branch::current_branch;
pub use refresh::refresh_existing_clone;
use stash::stash_dirty_clone_state;
