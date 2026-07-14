use super::{MergeResult, WorktreeManager};
use anyhow::Result;

impl WorktreeManager {
    /// Merge a worktree branch through an isolated, verified integration checkout.
    pub async fn merge(&self, name: &str) -> Result<MergeResult> {
        let branch = self.branch_for(name).await?;
        tracing::info!(worktree = %name, %branch, "Starting verified integration merge");
        self.verified_merge(name, &branch).await
    }
}
