use super::WorktreeManager;
use anyhow::{Context, Result};

impl WorktreeManager {
    pub(crate) async fn get_conflict_list(&self) -> Result<Vec<String>> {
        let output = tokio::process::Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=U"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to get conflict list")?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(String::from)
            .filter(|line| !line.is_empty())
            .collect())
    }

    pub(crate) async fn get_conflict_diffs(&self) -> Result<Vec<(String, String)>> {
        let mut diffs = Vec::new();
        for file in self.get_conflict_list().await? {
            let output = tokio::process::Command::new("git")
                .args(["diff", &file])
                .current_dir(&self.repo_path)
                .output()
                .await;
            if let Ok(output) = output {
                diffs.push((file, String::from_utf8_lossy(&output.stdout).to_string()));
            }
        }
        Ok(diffs)
    }
}
