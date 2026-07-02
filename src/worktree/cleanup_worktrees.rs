use super::WorktreeManager;
use anyhow::Result;

impl WorktreeManager {
    /// Remove tracked or discovered CodeTether worktrees without deleting branches.
    ///
    /// Dirty worktrees are preserved and remain tracked.
    pub async fn cleanup_worktrees_only(&self) -> Result<usize> {
        let infos = self.list().await;
        let mut removed = 0;
        for info in &infos {
            if self.remove_worktree(info).await.removed() {
                self.untrack(&info.name).await;
                removed += 1;
            }
        }
        tracing::info!(count = removed, "Cleaned up CodeTether worktrees");
        Ok(removed)
    }
}
