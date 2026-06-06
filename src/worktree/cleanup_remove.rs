use super::{WorktreeInfo, WorktreeManager};

impl WorktreeManager {
    pub(crate) async fn remove_worktree(&self, info: &WorktreeInfo) {
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
                    worktree = %info.name,
                    error = %String::from_utf8_lossy(&output.stderr),
                    "Git worktree remove failed, falling back to directory removal"
                );
                self.remove_worktree_dir(info).await;
            }
            Err(error) => {
                tracing::warn!(
                    worktree = %info.name,
                    error = %error,
                    "Failed to execute git worktree remove"
                );
                self.remove_worktree_dir(info).await;
            }
        }
    }

    async fn remove_worktree_dir(&self, info: &WorktreeInfo) {
        if let Err(error) = tokio::fs::remove_dir_all(&info.path).await {
            tracing::warn!(
                worktree = %info.name,
                error = %error,
                "Failed to remove worktree directory"
            );
        }
    }

    pub(crate) async fn untrack(&self, name: &str) {
        self.worktrees.lock().await.retain(|info| info.name != name);
    }
}
