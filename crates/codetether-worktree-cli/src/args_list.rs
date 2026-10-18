//! Command-line schema for worktree operations.

use clap::Parser;

/// Select the listing output format.
///
/// # Examples
/// ```
/// let args = codetether_worktree_cli::WorktreeListArgs { json: true };
/// assert!(args.json);
/// ```
#[derive(Parser, Debug)]
pub struct WorktreeListArgs {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}
