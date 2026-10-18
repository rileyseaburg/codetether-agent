use super::state::SwarmWorktrees;
use crate::task_result::TaskResult;

impl SwarmWorktrees {
    /// Commit and integrate a successful task, retaining failed worktrees for inspection.
    pub async fn finish(&self, index: usize, result: &mut TaskResult) {
        let Some(wt) = self.info(index).cloned() else {
            return;
        };
        if !result.success {
            tracing::info!(worktree_path = %wt.path.display(), "Keeping failed swarm worktree");
            return;
        }
        if let Err(error) = codetether_worktree::swarm::commit::prepare(&wt, &result.task_id).await
        {
            super::integrate::error(result, error);
            return;
        }
        let contributed =
            match codetether_worktree::swarm::branch::has_net_changes(&self.mgr, &wt).await {
                Ok(contributed) => contributed,
                Err(error) => return super::integrate::error(result, error),
            };
        super::integrate::apply(self, index, result, &wt, contributed).await;
    }
}
