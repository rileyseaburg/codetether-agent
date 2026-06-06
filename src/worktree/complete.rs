use super::{MergeResult, WorktreeManager};
use crate::provenance::{ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance};
use anyhow::{Result, anyhow};

impl WorktreeManager {
    /// Commit a merge after conflicts are resolved.
    pub async fn complete_merge(&self, name: &str, commit_msg: &str) -> Result<MergeResult> {
        let branch = self.branch_for(name).await?;
        let merge_head = self.merge_head_path().await?;
        if !tokio::fs::try_exists(&merge_head).await.unwrap_or(false) {
            return Err(anyhow!("Not in a merge state. Use merge() first."));
        }
        let provenance = ExecutionProvenance::for_operation("worktree", ExecutionOrigin::LocalCli);
        let output =
            git_commit_with_provenance(&self.repo_path, commit_msg, Some(&provenance)).await?;
        if !output.status.success() {
            return Err(anyhow!(
                "Failed to complete merge: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        tracing::info!(worktree = %name, branch = %branch, "Merge completed");
        Ok(MergeResult {
            success: true,
            aborted: false,
            conflicts: vec![],
            conflict_diffs: vec![],
            files_changed: self.count_merge_files_changed().await.unwrap_or(0),
            summary: format!("Merge completed: {}", commit_msg),
        })
    }

    /// Abort an in-progress merge for a known worktree.
    pub async fn abort_merge(&self, name: &str) -> Result<()> {
        let _branch = self.branch_for(name).await?;
        let merge_head = self.merge_head_path().await?;
        if !tokio::fs::try_exists(&merge_head).await.unwrap_or(false) {
            tracing::warn!("Not in a merge state, nothing to abort");
            return Ok(());
        }
        self.abort_merge_state().await;
        tracing::info!("Merge aborted");
        Ok(())
    }
}
