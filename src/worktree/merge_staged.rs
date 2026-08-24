use super::WorktreeManager;

impl WorktreeManager {
    /// Whether the index has staged changes relative to HEAD.
    ///
    /// Uses `git diff --cached --quiet`, which exits non-zero when staged
    /// changes exist. A failure to run git is treated as "no changes" so a
    /// no-op merge never produces a spurious commit error.
    pub(crate) async fn has_staged_changes(&self) -> bool {
        let status = crate::tool::git::process::output_refs(&self.repo_path, &["diff", "--cached", "--quiet"], false).await;
        match status {
            Ok(output) => !output.status.success(),
            Err(error) => {
                tracing::warn!(%error, "Failed to check staged changes; assuming none");
                false
            }
        }
    }
}