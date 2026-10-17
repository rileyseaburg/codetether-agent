//! Clean up worktrees after terminal status has been selected.
use super::*;
impl RalphLoop {
    pub(super) async fn cleanup_run_worktrees(&self) {
        // Clean up orphaned worktrees and branches
        if self.config.worktree_enabled {
            let mgr = WorktreeManager::for_repo(&self.state.working_dir);
            match mgr.cleanup_all().await {
                Ok(count) if count > 0 => {
                    info!(cleaned = count, "Cleaned up orphaned worktrees/branches");
                }
                Ok(_) => {}
                Err(e) => {
                    warn!(error = %e, "Failed to cleanup orphaned worktrees");
                }
            }
        }
    }
}
