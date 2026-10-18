//! JSON worktree cleanup preview.
use crate::CleanupArgs;
use codetether_worktree::WorktreeInfo;

pub(super) fn print(args: CleanupArgs, worktrees: &[WorktreeInfo]) -> anyhow::Result<()> {
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
