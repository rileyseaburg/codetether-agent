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
    pub async fn cleanup(&self, name: &str) -> Result<()> {
        let Some(info) = self.get(name).await else {
            return Ok(());
        };
        self.remove_worktree(&info).await;
        Self::delete_branch(&self.repo_path, &info.branch).await;
        self.untrack(name).await;
        Ok(())
    }

    /// Clean up all tracked or Git-discovered CodeTether worktrees.
    pub async fn cleanup_all(&self) -> Result<usize> {
        let infos = self.list().await;
        for info in &infos {
            self.remove_worktree(info).await;
        }
        let known: HashSet<String> = infos.iter().map(|info| info.branch.clone()).collect();
        let branches = self.codetether_branches().await.unwrap_or_default();
        let branch_only = branches
            .iter()
            .filter(|branch| !known.contains(*branch))
            .count();
        for branch in &branches {
            Self::delete_branch(&self.repo_path, branch).await;
        }
        self.worktrees.lock().await.clear();
        let count = infos.len() + branch_only;
        tracing::info!(count, "Cleaned up CodeTether worktrees/branches");
        Ok(count)
    }
}
