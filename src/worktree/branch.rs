use super::WorktreeManager;
#[path = "branch_delete.rs"]
mod delete;
use anyhow::{Context, Result};

#[cfg(test)]
mod tests;

impl WorktreeManager {
    pub(crate) async fn codetether_branches(&self) -> Result<Vec<String>> {
        let args = [
            "branch",
            "--list",
            "codetether/*",
            "--format=%(refname:short)",
        ];
        let output = crate::tool::git::process::output_refs(&self.repo_path, &args, false)
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
        delete::run(repo_path, branch).await
    }

    pub(crate) async fn delete_integrated_branch(repo: &std::path::Path, branch: &str) -> bool {
        crate::tool::git::process::output_refs(repo, &["branch", "-D", branch], true)
            .await
            .is_ok_and(|output| output.status.success())
    }
}
