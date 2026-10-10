use super::WorktreeManager;

#[path = "repair_benign.rs"]
mod repair_benign;
#[cfg(test)]
#[path = "repair_benign_tests.rs"]
mod repair_benign_tests;

impl WorktreeManager {
    pub(crate) async fn try_auto_repair(&self) {
        self.run_repair_step(["fetch", "--all", "--prune", "--tags"])
            .await;
        self.run_repair_step(["worktree", "prune"]).await;
        self.run_repair_step(["gc", "--prune=now"]).await;
    }

    pub(crate) async fn run_repair_step<const N: usize>(&self, args: [&str; N]) {
        let command = format!("git {}", args.join(" "));
        match tokio::process::Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                tracing::info!(
                    repo_path = %self.repo_path.display(),
                    command = %command,
                    "Git repair step succeeded"
                );
            }
            Ok(output) => {
                let details = Self::combined_output(&output.stdout, &output.stderr);
                self.log_repair_failure(&command, &details);
            }
            Err(error) => {
                tracing::warn!(
                    repo_path = %self.repo_path.display(),
                    command = %command,
                    error = %error,
                    "Failed to execute git repair step"
                );
            }
        }
    }
}
