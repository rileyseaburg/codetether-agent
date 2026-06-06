use super::super::TaskResult;
use super::state::SwarmWorktrees;

impl SwarmWorktrees {
    pub(in crate::tool::swarm_execute) async fn finish(
        &self,
        index: usize,
        result: &mut TaskResult,
    ) {
        let Some(wt) = self.info(index).cloned() else {
            return;
        };
        if !result.success {
            tracing::info!(worktree_path = %wt.path.display(), "Keeping failed swarm worktree");
            return;
        }
        match self.mgr.merge(&wt.name).await {
            Ok(merge) if merge.success => {
                result
                    .output
                    .push_str(&format!("\n\n--- Merge Result ---\n{}", merge.summary));
                if let Err(error) = self.mgr.cleanup(&wt.name).await {
                    tracing::warn!(error = %error, "Failed to cleanup swarm worktree");
                }
            }
            Ok(merge) => {
                result.success = false;
                result.error = Some(merge.summary.clone());
                result
                    .output
                    .push_str(&format!("\n\n--- Merge Failed ---\n{}", merge.summary));
            }
            Err(error) => mark_merge_error(result, error),
        }
    }
}

fn mark_merge_error(result: &mut TaskResult, error: anyhow::Error) {
    tracing::error!(error = %error, "Failed to merge swarm worktree");
    result.success = false;
    result.error = Some(format!("worktree merge failed: {error}"));
}
