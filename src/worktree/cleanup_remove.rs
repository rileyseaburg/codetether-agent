use super::remove_outcome::RemoveOutcome;
use super::{WorktreeInfo, WorktreeManager};

impl WorktreeManager {
    /// Remove a worktree unless it has uncommitted changes.
    ///
    /// Returns [`RemoveOutcome::RefusedDirty`] without touching the worktree
    /// when it is dirty, so callers skip branch deletion / untracking.
    pub(crate) async fn remove_worktree(&self, info: &WorktreeInfo) -> RemoveOutcome {
        if self.is_worktree_dirty(info).await {
            tracing::error!(
                worktree = %info.name, path = %info.path.display(),
                "Refusing to force-remove dirty worktree — commit or stash first."
            );
            return RemoveOutcome::RefusedDirty;
        }
        match tokio::process::Command::new("git")
            .args(["worktree", "remove", "--force"])
            .arg(&info.path)
            .current_dir(&self.repo_path)
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                tracing::info!(worktree = %info.name, "Removed git worktree");
            }
            Ok(output) => {
                tracing::warn!(
                    worktree = %info.name, error = %String::from_utf8_lossy(&output.stderr),
                    "Git worktree remove failed, falling back to directory removal"
                );
                self.remove_worktree_dir(info).await;
            }
            Err(error) => {
                tracing::warn!(worktree = %info.name, error = %error, "Failed git worktree remove");
                self.remove_worktree_dir(info).await;
            }
        }
        RemoveOutcome::Removed
    }

    async fn remove_worktree_dir(&self, info: &WorktreeInfo) {
        if let Err(error) = tokio::fs::remove_dir_all(&info.path).await {
            tracing::warn!(worktree = %info.name, error = %error, "Failed to remove worktree dir");
        }
    }

    pub(crate) async fn untrack(&self, name: &str) {
        self.worktrees.lock().await.retain(|info| info.name != name);
    }
}
