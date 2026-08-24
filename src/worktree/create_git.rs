use super::WorktreeManager;
use anyhow::{Context, Result};
use std::path::Path;

impl WorktreeManager {
    pub(crate) async fn add_worktree(
        &self,
        branch: &str,
        path: &Path,
        create_branch: bool,
    ) -> Result<std::process::Output> {
        let mut args = vec!["worktree".into(), "add".into()];
        if create_branch {
            args.extend(["-b".into(), branch.into(), path.display().to_string()]);
        } else {
            args.extend([path.display().to_string(), branch.into()]);
        }
        crate::tool::git::process::output(&self.repo_path, &args, &[], true)
            .await
            .context("Failed to execute git worktree add")
    }
}
