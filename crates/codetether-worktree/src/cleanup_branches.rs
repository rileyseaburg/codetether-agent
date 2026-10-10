//! Branch cleanup helper for [`super::WorktreeManager::cleanup_all`].
//!
//! Split out to keep `cleanup.rs` within the module line budget.

use super::{WorktreeInfo, WorktreeManager};
use std::collections::HashSet;

impl WorktreeManager {
    /// Delete CodeTether branches except those of refused (dirty) worktrees.
    pub(crate) async fn delete_orphan_branches(
        &self,
        infos: &[WorktreeInfo],
        refused: &HashSet<String>,
    ) -> usize {
        let keep: HashSet<String> = infos
            .iter()
            .filter(|i| refused.contains(&i.name))
            .map(|i| i.branch.clone())
            .collect();
        let branches = self.codetether_branches().await.unwrap_or_default();
        let mut deleted = 0;
        for branch in &branches {
            if keep.contains(branch) {
                continue;
            }
            deleted += usize::from(Self::delete_branch(&self.repo_path, branch).await);
        }
        deleted
    }
}
