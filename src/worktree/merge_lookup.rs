use super::WorktreeManager;
use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;

impl WorktreeManager {
    pub(crate) async fn branch_for(&self, name: &str) -> Result<String> {
        self.list()
            .await
            .into_iter()
            .find(|info| info.name == name)
            .map(|info| info.branch)
            .ok_or_else(|| anyhow!("Worktree not found: {}", name))
    }

    pub(crate) async fn merge_head_path(&self) -> Result<PathBuf> {
        let output = tokio::process::Command::new("git")
            .args(["rev-parse", "--git-path", "MERGE_HEAD"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to determine git merge metadata path")?;
        if !output.status.success() {
            return Err(anyhow!(
                "Failed to resolve merge metadata path: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        let merge_head = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if merge_head.is_empty() {
            return Err(anyhow!("Git returned an empty MERGE_HEAD path"));
        }
        let path = PathBuf::from(&merge_head);
        Ok(if path.is_absolute() {
            path
        } else {
            self.repo_path.join(path)
        })
    }

    pub(crate) async fn count_merge_files_changed(&self) -> Result<usize> {
        let output = tokio::process::Command::new("git")
            .args(["diff", "--name-only", "HEAD~1", "HEAD"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to count changed files")?;
        Ok(String::from_utf8_lossy(&output.stdout).lines().count())
    }
}
