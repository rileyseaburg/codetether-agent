//! Failure-driven integrity recovery for worktree creation.

use super::WorktreeManager;
use anyhow::Result;
use std::path::Path;

impl WorktreeManager {
    pub(crate) async fn create_or_recover(
        &self,
        branch: &str,
        path: &Path,
        start_point: Option<&str>,
    ) -> Result<std::process::Output> {
        let mut output = self.add_worktree(branch, path, true, start_point).await?;
        let details = Self::combined_output(&output.stdout, &output.stderr);
        if !output.status.success() && Self::looks_like_object_corruption(&details) {
            self.ensure_repo_integrity().await?;
            output = self.add_worktree(branch, path, true, start_point).await?;
        }
        // Explicit revisions must never reuse an unrelated existing branch.
        if output.status.success() || start_point.is_some() {
            return Ok(output);
        }
        self.add_worktree(branch, path, false, None).await
    }
}
