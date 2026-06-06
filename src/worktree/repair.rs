use super::WorktreeManager;

impl WorktreeManager {
    pub(crate) async fn try_auto_repair(&self) {
        self.run_repair_step(["fetch", "--all", "--prune", "--tags"])
            .await;
        self.run_repair_step(["worktree", "prune"]).await;
        self.run_repair_step(["gc", "--prune=now"]).await;
    }

    pub(crate) async fn run_repair_step<const N: usize>(&self, args: [&str; N]) {
        match tokio::process::Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                tracing::info!(
                    repo_path = %self.repo_path.display(),
                    command = %format!("git {}", args.join(" ")),
                    "Git repair step succeeded"
                );
            }
            Ok(output) => {
                let details = Self::combined_output(&output.stdout, &output.stderr);
                tracing::warn!(
                    repo_path = %self.repo_path.display(),
                    command = %format!("git {}", args.join(" ")),
                    error = %Self::summarize_git_output(&details),
                    "Git repair step failed"
                );
            }
            Err(error) => {
                tracing::warn!(
                    repo_path = %self.repo_path.display(),
                    command = %format!("git {}", args.join(" ")),
                    error = %error,
                    "Failed to execute git repair step"
                );
            }
        }
    }
}
