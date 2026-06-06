use super::{
    WorktreeManager,
    sync_git::{git_best_effort, git_output},
};
use anyhow::Result;

impl WorktreeManager {
    pub(crate) fn reset_lingering_merge(&self) -> Result<()> {
        let unmerged = git_output(&self.repo_path, &["diff", "--name-only", "--diff-filter=U"])?;
        if String::from_utf8_lossy(&unmerged.stdout).trim().is_empty() {
            return Ok(());
        }
        tracing::warn!("Resetting unmerged index entries before merge");
        git_best_effort(&self.repo_path, &["merge", "--abort"]);
        git_best_effort(&self.repo_path, &["reset", "HEAD", "--"]);
        git_best_effort(&self.repo_path, &["checkout", "--", "."]);
        Ok(())
    }
}
