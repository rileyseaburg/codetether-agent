//! Command-line schema for worktree operations.

use clap::Parser;

/// Select a worktree and VS Code window behavior.
///
/// # Examples
/// ```
/// let args = codetether_worktree_cli::WorktreeOpenArgs { name: "feature".into(), reuse_window: false };
/// assert!(!args.reuse_window);
/// ```
#[derive(Parser, Debug)]
pub struct WorktreeOpenArgs {
    /// Worktree name (as shown by `worktree list`)
    pub name: String,

    /// Reuse the current VS Code window instead of opening a new one
    #[arg(short, long)]
    pub reuse_window: bool,
}
