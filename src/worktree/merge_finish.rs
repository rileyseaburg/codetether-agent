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
        // A --no-commit merge of a branch with no new commits (e.g. a sub-agent
        // that made zero file changes) stages nothing, so the follow-up commit
        // would fail with "nothing to commit". Treat that as a successful no-op
        // instead of surfacing a spurious "Git merge commit failed" error.
        if !self.has_staged_changes().await {
            tracing::info!(worktree = %name, branch = %branch, "Merge is a no-op (nothing to commit)");
            self.pop_stash_if(stashed);
            return Ok(MergeResult {
                success: true,
                aborted: false,
                conflicts: vec![],
                conflict_diffs: vec![],
                files_changed: 0,
                summary: format!("Branch '{branch}' had no changes to merge"),
            });
        }
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
