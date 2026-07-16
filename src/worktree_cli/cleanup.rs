//! `worktree cleanup` preview and apply handler.

use codetether_agent::{
    cli::worktree_args::WorktreeCleanupArgs,
    worktree::{WorktreeManager, maintenance::WorktreeCleanupState},
};

/// Preview cleanup by default, applying only when explicitly requested.
pub async fn run(args: WorktreeCleanupArgs, mgr: &WorktreeManager) -> anyhow::Result<()> {
    let report = if args.apply {
        mgr.apply_merged_cleanup(&args.base, &args.root).await?
    } else {
        mgr.plan_merged_cleanup(&args.base, &args.root).await?
    };
    super::cleanup_report::print(&report, args.json)?;
    let failures = report
        .entries
        .iter()
        .filter(|entry| entry.state == WorktreeCleanupState::RemovalFailed)
        .count();
    if failures > 0 {
        anyhow::bail!("{failures} worktree removal(s) failed; see the report above");
    }
    Ok(())
}
