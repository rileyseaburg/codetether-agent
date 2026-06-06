use super::WorktreeManager;
use anyhow::{Context, Result, anyhow};

impl WorktreeManager {
    /// Verify repository object integrity before worktree operations.
    pub async fn ensure_repo_integrity(&self) -> Result<()> {
        let first_check = self.run_repo_fsck().await?;
        if first_check.status.success() {
            return Ok(());
        }
        let first_output = Self::combined_output(&first_check.stdout, &first_check.stderr);
        if !Self::looks_like_object_corruption(&first_output) {
            return Err(anyhow!(
                "Git repository preflight failed: {}",
                Self::summarize_git_output(&first_output)
            ));
        }
        tracing::warn!(
            repo_path = %self.repo_path.display(),
            issue = %Self::summarize_git_output(&first_output),
            "Detected git object corruption; attempting automatic repair"
        );
        self.try_auto_repair().await;
        let second_check = self.run_repo_fsck().await?;
        if second_check.status.success() {
            tracing::info!(repo_path = %self.repo_path.display(), "Git repository integrity restored");
            return Ok(());
        }
        let second_output = Self::combined_output(&second_check.stdout, &second_check.stderr);
        Err(Self::integrity_error_message(
            &self.repo_path,
            &second_output,
        ))
    }

    pub(crate) async fn ensure_repo_integrity_once(&self) -> Result<()> {
        let mut checked = self.integrity_checked.lock().await;
        if *checked {
            return Ok(());
        }
        self.ensure_repo_integrity().await?;
        *checked = true;
        Ok(())
    }

    pub(crate) async fn run_repo_fsck(&self) -> Result<std::process::Output> {
        tokio::process::Command::new("git")
            .args(["fsck", "--full", "--no-dangling"])
            .current_dir(&self.repo_path)
            .output()
            .await
            .context("Failed to execute git fsck --full --no-dangling")
    }
}
