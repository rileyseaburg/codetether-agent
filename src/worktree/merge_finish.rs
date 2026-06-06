use super::{MergeResult, WorktreeManager};
use crate::provenance::{ExecutionOrigin, ExecutionProvenance, git_commit_with_provenance};
use anyhow::{Result, anyhow};

impl WorktreeManager {
    pub(crate) async fn finish_successful_merge(
        &self,
        name: &str,
        branch: &str,
        stashed: bool,
    ) -> Result<MergeResult> {
        let commit_msg = format!("Merge branch '{}' into current branch", branch);
        let provenance = ExecutionProvenance::for_operation("worktree", ExecutionOrigin::LocalCli);
        let output =
            git_commit_with_provenance(&self.repo_path, &commit_msg, Some(&provenance)).await?;
        if !output.status.success() {
            self.pop_stash_if(stashed);
            return Err(anyhow!(
                "Git merge commit failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        tracing::info!(worktree = %name, branch = %branch, "Git merge successful");
        self.pop_stash_if(stashed);
        Ok(MergeResult {
            success: true,
            aborted: false,
            conflicts: vec![],
            conflict_diffs: vec![],
            files_changed: self.count_merge_files_changed().await.unwrap_or(0),
            summary: commit_msg,
        })
    }
}
