//! Command-line schema for worktree operations.

use clap::Parser;
use std::path::PathBuf;

/// Select the workspace file destination and optional editor launch.
///
/// # Examples
/// ```
/// let args = codetether_worktree_cli::WorktreeWorkspaceArgs { dest: "project.code-workspace".into(), open: false };
/// assert!(!args.open);
/// ```
#[derive(Parser, Debug)]
pub struct WorktreeWorkspaceArgs {
    /// Destination path for the `.code-workspace` file
    #[arg(long, default_value = "codetether.code-workspace")]
    pub dest: PathBuf,

    /// Open the generated workspace in VS Code after writing it
    #[arg(long)]
    pub open: bool,
}
