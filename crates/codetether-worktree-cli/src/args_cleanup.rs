//! Command-line schema for worktree operations.

use clap::Parser;
use std::path::PathBuf;

/// Options for repository-wide safe worktree cleanup.
///
/// # Examples
/// ```
/// let args = codetether_worktree_cli::WorktreeCleanupArgs { base: "main".into(), root: vec![], apply: false, json: true };
/// assert!(!args.apply);
/// ```
#[derive(Parser, Debug)]
pub struct WorktreeCleanupArgs {
    /// Branch or commit that candidates must already be merged into.
    #[arg(long, default_value = "main")]
    pub base: String,
    /// Limit cleanup to worktrees beneath one or more directories.
    #[arg(long)]
    pub root: Vec<PathBuf>,
    /// Apply cleanup; omission produces a read-only preview.
    #[arg(long)]
    pub apply: bool,
    /// Output the complete cleanup report as JSON.
    #[arg(long)]
    pub json: bool,
}
