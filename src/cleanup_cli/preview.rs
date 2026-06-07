use codetether_agent::{cli::CleanupArgs, worktree::WorktreeInfo};

pub fn print(args: CleanupArgs, worktrees: &[WorktreeInfo]) -> anyhow::Result<()> {
    if args.json {
        return json(args, worktrees);
    }
    println!("# Cleanup Preview (Dry Run)\n");
    if worktrees.is_empty() {
        println!("No orphaned worktrees or branches found.");
    } else {
        println!("Found {} worktree(s) to clean:\n", worktrees.len());
        for wt in worktrees {
            println!("- **{}** -> {}", wt.branch, wt.path.display());
        }
        println!("\nRun without --dry-run to delete {}.", scope(args));
    }
    Ok(())
}

fn json(args: CleanupArgs, worktrees: &[WorktreeInfo]) -> anyhow::Result<()> {
    let worktrees = worktrees
        .iter()
        .map(|wt| {
            serde_json::json!({
                "id": wt.name,
                "branch": wt.branch,
                "path": wt.path.display().to_string(),
            })
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "dry_run": true,
            "worktrees_only": args.worktrees_only,
            "worktrees_found": worktrees.len(),
            "worktrees": worktrees,
        }))?
    );
    Ok(())
}

fn scope(args: CleanupArgs) -> &'static str {
    if args.worktrees_only {
        "worktrees only"
    } else {
        "worktrees and branches"
    }
}
