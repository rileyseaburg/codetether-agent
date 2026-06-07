use clap::Parser;

#[derive(Parser, Debug)]
pub struct CleanupArgs {
    /// Dry run - show what would be cleaned up without deleting
    #[arg(short, long)]
    pub dry_run: bool,

    /// Clean up worktrees only, preserving branches
    #[arg(long, conflicts_with = "artifacts")]
    pub worktrees_only: bool,

    /// Clean worktree target directories, preserving worktrees and branches
    #[arg(long, conflicts_with = "worktrees_only")]
    pub artifacts: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}
