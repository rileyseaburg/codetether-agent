use super::{WorktreeManager, sync_git::git_output};
use anyhow::Result;

impl WorktreeManager {
    pub(crate) fn stash_dirty_worktree(&self, name: &str) -> Result<bool> {
        if !self.has_dirty_worktree()? {
            return Ok(false);
        }
        let before = self.current_stash_oid()?;
        let marker = format!("codetether-worktree-merge-{name}");
        let output = git_output(
            &self.repo_path,
            &["stash", "push", "--include-untracked", "-m", &marker],
        )?;
        if !output.status.success() {
            tracing::warn!(
                error = %String::from_utf8_lossy(&output.stderr),
                "Stash failed before worktree merge"
            );
            return Ok(false);
        }
        Ok(self.current_stash_oid()? != before)
    }

    pub(crate) fn pop_stash_if(&self, stashed: bool) {
        if !stashed {
            return;
        }
        match git_output(&self.repo_path, &["stash", "pop"]) {
            Ok(output) if output.status.success() => {}
            Ok(output) => tracing::warn!(
                error = %String::from_utf8_lossy(&output.stderr),
                "Failed to pop CodeTether merge stash"
            ),
            Err(error) => tracing::warn!(error = %error, "Failed to pop CodeTether merge stash"),
        }
    }

    fn has_dirty_worktree(&self) -> Result<bool> {
        let output = git_output(&self.repo_path, &["status", "--porcelain"])?;
        Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
    }

    fn current_stash_oid(&self) -> Result<Option<String>> {
        let output = git_output(&self.repo_path, &["rev-parse", "--verify", "refs/stash"])?;
        if !output.status.success() {
            return Ok(None);
        }
        Ok(Some(
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        ))
    }
}
