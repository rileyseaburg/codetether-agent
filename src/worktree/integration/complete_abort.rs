use crate::worktree::WorktreeManager;

impl WorktreeManager {
    pub(crate) async fn abort_merge_state(&self) {
        match crate::tool::git::process::output_refs(&self.repo_path, &["merge", "--abort"], true)
            .await
        {
            Ok(output) if output.status.success() => {}
            Ok(output) => tracing::warn!(
                stderr = %String::from_utf8_lossy(&output.stderr),
                "Failed to abort merge state"
            ),
            Err(error) => tracing::warn!(%error, "Failed to run git merge --abort"),
        }
    }
}
