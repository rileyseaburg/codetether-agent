use super::WorktreeManager;
use anyhow::{Context, Result};

#[cfg(test)]
mod tests;

impl WorktreeManager {
    pub(crate) async fn codetether_branches(&self) -> Result<Vec<String>> {
        let output = tokio::process::Command::new("git")
            .args([
                "branch",
                "--list",
                "codetether/*",
                "--format=%(refname:short)",
            ])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to list CodeTether branches")?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect())
    }

    pub(crate) async fn delete_branch(repo_path: &std::path::Path, branch: &str) -> bool {
        match tokio::process::Command::new("git")
            .args(["branch", "-d", branch])
            .current_dir(repo_path)
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                tracing::info!(branch, "Deleted merged worktree branch");
                true
            }
            Ok(output) => {
                tracing::warn!(branch, error = %String::from_utf8_lossy(&output.stderr),
                    "Preserving unmerged worktree branch");
                false
            }
            Err(error) => {
                tracing::warn!(branch, error = %error, "Worktree branch delete failed");
                false
            }
        }
    }
}
