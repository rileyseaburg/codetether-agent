//! Clean up worktrees after terminal status has been selected.
use super::*;
pub(crate) async fn cleanup_run_worktrees<H: Host>(host: &H) {
    // Clean up orphaned worktrees and branches
    if host.config().worktree_enabled {
        let mgr = WorktreeManager::for_repo(&host.state().working_dir);
        match mgr.cleanup_all().await {
            Ok(count) if count > 0 => {
                info!(target: "codetether_agent::ralph::ralph_loop::lifecycle_cleanup", cleaned = count, "Cleaned up orphaned worktrees/branches");
            }
            Ok(_) => {}
            Err(e) => {
                warn!(target: "codetether_agent::ralph::ralph_loop::lifecycle_cleanup", error = %e, "Failed to cleanup orphaned worktrees");
            }
        }
    }
}
