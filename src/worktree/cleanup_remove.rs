use super::{WorktreeInfo, WorktreeManager};
mod outcome;
pub(crate) use outcome::RemoveOutcome;

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
                tracing::error!(
                    worktree = %info.name, error = %String::from_utf8_lossy(&output.stderr),
                    "Git worktree removal failed; preserving checkout"
                );
                return RemoveOutcome::Failed;
            }
            Err(error) => {
                tracing::error!(worktree = %info.name, error = %error, "Git worktree removal failed");
                return RemoveOutcome::Failed;
            }
        }
        RemoveOutcome::Removed
    }
}
