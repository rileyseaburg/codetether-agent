use super::{WorktreeInfo, WorktreeManager};
use anyhow::Result;
use std::collections::HashSet;

impl WorktreeManager {
    /// Get information about a worktree.
    #[allow(dead_code)]
    pub async fn get(&self, name: &str) -> Option<WorktreeInfo> {
        self.list().await.into_iter().find(|info| info.name == name)
    }

    /// Clean up a specific worktree and its local branch.
    ///
    /// A dirty worktree is left intact (branch kept, still tracked) so
    /// uncommitted work is preserved.
    pub async fn cleanup(&self, name: &str) -> Result<()> {
        let Some(info) = self.get(name).await else {
            return Ok(());
        };
        if self.remove_worktree(&info).await.removed() {
            Self::delete_branch(&self.repo_path, &info.branch).await;
            self.untrack(name).await;
        }
        Ok(())
    }

    /// Clean up all tracked or Git-discovered CodeTether worktrees.
    ///
    /// Dirty worktrees (and their branches) are preserved.
    pub async fn cleanup_all(&self) -> Result<usize> {
        let infos = self.list().await;
        let refused = self.remove_all(&infos).await;
        let count = self.delete_orphan_branches(&infos, &refused).await;
        self.worktrees
            .lock()
            .await
            .retain(|i| refused.contains(&i.name));
        tracing::info!(count, "Cleaned up CodeTether worktrees/branches");
        Ok(count)
    }

    /// Remove every worktree, returning the names that were refused (dirty).
    async fn remove_all(&self, infos: &[WorktreeInfo]) -> HashSet<String> {
        let mut refused = HashSet::new();
        for info in infos {
            if !self.remove_worktree(info).await.removed() {
                refused.insert(info.name.clone());
            }
        }
        refused
    }
}
