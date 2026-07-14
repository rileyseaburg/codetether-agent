use super::{IntegrationDir, apply, git, merge, outcome::Outcome, result};
use crate::worktree::{MergeResult, WorktreeManager};
use anyhow::Result;

impl WorktreeManager {
    pub(crate) async fn verified_merge(&self, name: &str, branch: &str) -> Result<MergeResult> {
        let stashed = self.stash_dirty_worktree(name)?;
        let outcome = self.integrate(name, branch);
        self.pop_stash_if(stashed);
        outcome
    }

    fn integrate(&self, name: &str, branch: &str) -> Result<MergeResult> {
        let integration = IntegrationDir::create(&self.repo_path, &self.base_dir, name)?;
        let files = match merge::merge(integration.path(), branch)? {
            Outcome::Merged(files) => files,
            Outcome::Conflicted(conflicts) => return Ok(result::conflict(conflicts)),
        };
        if files.is_empty() {
            return Ok(result::noop(branch));
        }
        let commit = git::text(integration.path(), &["rev-parse", "HEAD"])?;
        apply::fast_forward(&self.repo_path, &commit)?;
        tracing::info!(worktree = %name, %branch, %commit, files = files.len(),
            "Verified integration merge fast-forwarded");
        Ok(result::success(branch, commit, files.len()))
    }
}
