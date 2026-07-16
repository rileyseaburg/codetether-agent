//! Git worktree management for isolated agent execution.
//!
//! The module keeps worktree creation, discovery, cleanup, and merge handling
//! split into small files so each file owns one part of the lifecycle.

mod artifact_collect;
mod artifacts;
mod branch;
mod cleanup;
mod cleanup_branches;
mod cleanup_remove;
#[cfg(test)]
mod cleanup_remove_tests;
mod cleanup_worktrees;
mod complete;
mod create;
mod create_git;
mod dirty_check;
mod discover;
mod discover_parse;
#[cfg(test)]
mod discover_parse_tests;
mod info;
mod integration;
mod integrity;
mod integrity_error;
pub mod maintenance;
mod manager;
mod merge;
mod merge_lookup;
#[cfg(test)]
mod merge_staged_tests;
mod node_dependencies;
mod output;
#[cfg(test)]
mod output_tests;
mod repair;
mod stash;
mod sync_git;
mod tui_active;
mod validate;
mod vscode_auto;
#[cfg(test)]
mod vscode_auto_tests;
mod vscode_open;
#[cfg(test)]
mod vscode_open_tests;
mod vscode_open_workspace;
mod vscode_prompt;
mod vscode_workspace;
#[cfg(test)]
mod vscode_workspace_tests;

pub use info::{MergeResult, WorktreeInfo};
pub use manager::WorktreeManager;
pub use tui_active::{is_tui_active, set_tui_active};
