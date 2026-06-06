use super::{MergeResult, WorktreeManager};
use anyhow::Result;

impl WorktreeManager {
    /// Merge a worktree branch back into the current branch.
    pub async fn merge(&self, name: &str) -> Result<MergeResult> {
        let branch = self.branch_for(name).await?;
        self.reset_lingering_merge()?;
        let stashed = self.stash_dirty_worktree(name)?;
        tracing::info!(worktree = %name, branch = %branch, "Starting git merge");
        let mut output = self.run_merge(&branch, false).await?;
        if !output.status.success() && Self::merge_output_has_conflict(&output) {
            tracing::warn!(
                worktree = %name,
                "Merge has conflicts; retrying with incoming branch preference"
            );
            self.abort_merge_state().await;
            output = self.run_merge(&branch, true).await?;
        }
        if output.status.success() {
            self.finish_successful_merge(name, &branch, stashed).await
        } else {
            self.failed_merge_result(output, stashed).await
        }
    }
}
