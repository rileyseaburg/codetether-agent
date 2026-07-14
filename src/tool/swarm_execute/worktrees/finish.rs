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
        if let Err(error) = crate::swarm::worktree_commit::prepare(&wt, &result.task_id).await {
            super::integrate::error(result, error);
            return;
        }
        let contributed = match crate::swarm::worktree_branch::has_net_changes(&self.mgr, &wt).await
        {
            Ok(contributed) => contributed,
            Err(error) => return super::integrate::error(result, error),
        };
        super::integrate::apply(self, index, result, &wt, contributed).await;
    }
}
