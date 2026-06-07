mod artifact_preview;
mod preview;
mod report;

use codetether_agent::{cli::CleanupArgs, worktree::WorktreeManager};

pub async fn run(args: CleanupArgs) -> anyhow::Result<()> {
    let mgr = WorktreeManager::for_repo(std::env::current_dir()?);
    if args.artifacts {
        return artifacts(args, &mgr).await;
    }
    let worktrees = mgr.list().await;
    if args.dry_run {
        return preview::print(args, &worktrees);
    }
    let count = if args.worktrees_only {
        mgr.cleanup_worktrees_only().await?
    } else {
        mgr.cleanup_all().await?
    };
    report::done(args, count);
    Ok(())
}

async fn artifacts(args: CleanupArgs, mgr: &WorktreeManager) -> anyhow::Result<()> {
    if args.dry_run {
        let dirs = mgr.build_artifact_dirs().await?;
        return artifact_preview::print(args, &dirs);
    }
    let count = mgr.cleanup_build_artifacts().await?;
    report::artifacts(args, count);
    Ok(())
}
