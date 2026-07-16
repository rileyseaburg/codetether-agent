//! Safe cleanup planning for every worktree registered with a repository.
//!
//! Cleanup is based on Git metadata and commit ancestry, not directory names.
//! [`WorktreeCleanupReport`] records why every checkout is removed or kept.
//!
//! # Usage
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::worktree::WorktreeManager;
//! use std::path::PathBuf;
//!
//! let manager = WorktreeManager::for_repo("/srv/project");
//! let roots = vec![PathBuf::from("/srv/project-worktrees")];
//! let report = manager.plan_merged_cleanup("main", &roots).await?;
//! assert!(!report.applied);
//! # Ok::<(), anyhow::Error>(())
//! # }).unwrap();
//! ```

mod apply;
mod checks;
mod decision;
mod discover;
mod entry;
mod inspect;
mod parse;
mod plan;
mod plan_support;
mod record;
mod remove;
mod report;
mod roots;
mod state;

#[cfg(test)]
mod tests;

pub use entry::WorktreeCleanupEntry;
pub use report::WorktreeCleanupReport;
pub use state::WorktreeCleanupState;
