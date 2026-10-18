//! Human-readable worktree cleanup previews.

use crate::CleanupArgs;
use codetether_worktree::WorktreeInfo;

pub fn print(args: CleanupArgs, worktrees: &[WorktreeInfo]) -> anyhow::Result<()> {
    if args.json {
        return super::preview_json::print(args, worktrees);
    }
    println!("# Cleanup Preview (Dry Run)\n");
    if worktrees.is_empty() {
        println!("No orphaned worktrees or branches found.");
    } else {
        println!("Found {} worktree(s) to clean:\n", worktrees.len());
        for wt in worktrees {
            println!("- **{}** -> {}", wt.branch, wt.path.display());
        }
        println!(
            "\nRun without --dry-run to delete {}.",
            super::preview_scope::scope(args)
        );
    }
    Ok(())
}
