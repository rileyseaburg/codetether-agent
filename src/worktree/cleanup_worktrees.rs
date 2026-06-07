use super::WorktreeManager;
use anyhow::Result;

impl WorktreeManager {
    /// Remove tracked or discovered CodeTether worktrees without deleting branches.
    pub async fn cleanup_worktrees_only(&self) -> Result<usize> {
        let infos = self.list().await;
        for info in &infos {
            self.remove_worktree(info).await;
            self.untrack(&info.name).await;
        }
        tracing::info!(count = infos.len(), "Cleaned up CodeTether worktrees");
        Ok(infos.len())
    }
}
