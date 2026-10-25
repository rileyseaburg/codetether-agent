//! Retire a worker checkout once its patch is safely delivered and recorded.

use crate::WorktreeManager;
use anyhow::Result;

impl WorktreeManager {
    /// Remove a delivered worker checkout and its branch.
    ///
    /// Delivery never merges, so the worker branch is not an ancestor of the
    /// target and `git branch -d` would refuse it. The branch is force-deleted
    /// only after its worktree is removed cleanly; the delivered patch and
    /// head commit remain in the SQLite delivery ledger. A dirty worktree is
    /// left intact.
    ///
    /// # Errors
    ///
    /// Returns an error only if Git cannot be executed.
    pub async fn discard_delivered(&self, name: &str) -> Result<()> {
        let Some(info) = self.get(name).await else {
            return Ok(());
        };
        if !self.remove_worktree(&info).await.removed() {
            return Ok(());
        }
        let output = tokio::process::Command::new("git")
            .args(["branch", "-D", &info.branch])
            .current_dir(&self.repo_path)
            .output()
            .await?;
        if !output.status.success() {
            tracing::warn!(branch = %info.branch,
                error = %String::from_utf8_lossy(&output.stderr), "Delivered branch kept");
        }
        self.untrack(name).await;
        Ok(())
    }
}
