//! CLI orchestration for managed worktree and build-artifact cleanup.
//!
//! [CleanupArgs] selects preview or mutation mode and the cleanup scope.
//! [run] delegates filesystem/Git work to the shared worktree manager and
//! prints the existing human-readable or JSON results.
mod args;
pub use args::CleanupArgs;
mod artifact_preview;
mod preview;
mod report;

use codetether_worktree::WorktreeManager;

/// Execute cleanup or print a preview according to the supplied options.
///
/// # Arguments
/// * `args` - Scope, dry-run and output format settings.
/// # Returns
/// Unit after printing the preview or cleanup result.
/// # Errors
/// Returns current-directory, worktree-manager or serialization errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_cleanup_cli::run(codetether_cleanup_cli::CleanupArgs {
///     dry_run: true, worktrees_only: true, artifacts: false, json: true,
/// }).await
/// # }
/// ```
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

mod preview_json;
mod preview_scope;
