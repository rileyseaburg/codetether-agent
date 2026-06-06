use super::WorktreeManager;
use anyhow::{Context, Result};
use std::process::Output;

impl WorktreeManager {
    pub(crate) async fn run_merge(&self, branch: &str, prefer_theirs: bool) -> Result<Output> {
        let mut args = vec!["merge", "--no-ff", "--no-commit"];
        if prefer_theirs {
            args.extend(["-X", "theirs"]);
        }
        args.push(branch);
        tokio::process::Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git merge")
    }

    pub(crate) async fn abort_merge_state(&self) {
        let _ = tokio::process::Command::new("git")
            .args(["merge", "--abort"])
            .current_dir(&self.repo_path)
            .output()
            .await;
    }

    pub(crate) fn merge_output_has_conflict(output: &Output) -> bool {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        stderr.contains("CONFLICT") || stdout.contains("CONFLICT")
    }
}
