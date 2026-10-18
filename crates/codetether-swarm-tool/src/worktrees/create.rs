use super::state::SwarmWorktrees;
use codetether_worktree::WorktreeManager;
use futures::stream::{self, StreamExt};
use std::sync::Arc;

#[path = "create_one.rs"]
mod create_one;
#[path = "create_tasks.rs"]
mod create_tasks;

const PARALLELISM: usize = 8;

impl SwarmWorktrees {
    /// Allocate isolated worktrees in task order with at most eight parallel creations.
    pub async fn create(
        repo: &std::path::Path,
        tasks: &[(String, String, Option<String>, Option<bool>)],
    ) -> Self {
        let mgr = Arc::new(WorktreeManager::for_repo(repo).without_vscode_auto_open());
        let (prepared, expects_changes, verification) = create_tasks::prepare(tasks);
        let infos = stream::iter(prepared.into_iter().map(|(name, task)| {
            let mgr = Arc::clone(&mgr);
            async move { create_one::run(&mgr, &name, &task).await }
        }))
        .buffered(PARALLELISM)
        .collect()
        .await;
        Self {
            mgr,
            infos,
            expects_changes,
            verification,
        }
    }
}
