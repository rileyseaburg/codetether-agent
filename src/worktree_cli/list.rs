//! `worktree list` handler.

use codetether_agent::{cli::WorktreeListArgs, worktree::WorktreeManager};

/// List discovered worktrees as a table or JSON.
pub async fn run(args: WorktreeListArgs, mgr: &WorktreeManager) -> anyhow::Result<()> {
    let worktrees = mgr.list().await;
    if args.json {
        let rows: Vec<_> = worktrees
            .iter()
            .map(|w| {
                serde_json::json!({
                    "name": w.name,
                    "path": w.path,
                    "branch": w.branch,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&rows)?);
        return Ok(());
    }
    if worktrees.is_empty() {
        println!("No worktrees found.");
        return Ok(());
    }
    for w in &worktrees {
        println!("{}\t{}\t{}", w.name, w.branch, w.path.display());
    }
    Ok(())
}
