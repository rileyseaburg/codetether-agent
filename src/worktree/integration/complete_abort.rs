use crate::worktree::WorktreeManager;

impl WorktreeManager {
    pub(crate) async fn abort_merge_state(&self) {
        match tokio::process::Command::new("git")
            .args(["merge", "--abort"])
            .current_dir(&self.repo_path)
            .output()
            .await
        {
            Ok(output) if output.status.success() => {}
            Ok(output) => tracing::warn!(
                stderr = %String::from_utf8_lossy(&output.stderr),
                "Failed to abort merge state"
            ),
            Err(error) => tracing::warn!(%error, "Failed to run git merge --abort"),
        }
    }
}
