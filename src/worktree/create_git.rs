//! Git command execution for new and legacy reused worktree branches.

use super::WorktreeManager;
use anyhow::{Context, Result};
use std::path::Path;

impl WorktreeManager {
    pub(crate) async fn add_worktree(
        &self,
        branch: &str,
        path: &Path,
        create_branch: bool,
        start_point: Option<&str>,
    ) -> Result<std::process::Output> {
        let mut command = tokio::process::Command::new("git");
        command.arg("worktree").arg("add");
        if create_branch {
            command.arg("-b").arg(branch).arg(path);
            if let Some(start_point) = start_point {
                command.arg(start_point);
            }
        } else {
            command.arg(path).arg(branch);
        }
        command
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git worktree add")
    }
}
