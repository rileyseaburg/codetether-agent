//! Allocation of one task-specific swarm worktree.

use crate::swarm::SubTask;
use crate::worktree::{WorktreeInfo, WorktreeManager};
use std::sync::Arc;

pub(super) async fn run(
    manager: &Arc<WorktreeManager>,
    name: &str,
    task: &SubTask,
) -> Option<WorktreeInfo> {
    if !task.needs_worktree() {
        return None;
    }
    let slug = format!("swarm_{}", uuid::Uuid::new_v4().simple());
    match manager.create(&slug).await {
        Ok(worktree) => prepare(manager, name, worktree),
        Err(error) => {
            tracing::warn!(task = %name, %error, "swarm worktree create failed");
            None
        }
    }
}

fn prepare(manager: &WorktreeManager, name: &str, worktree: WorktreeInfo) -> Option<WorktreeInfo> {
    if let Err(error) = manager.inject_workspace_stub(&worktree.path) {
        tracing::warn!(task = %name, path = %worktree.path.display(), %error,
            "swarm workspace isolation setup failed");
        return None;
    }
    Some(worktree)
}
