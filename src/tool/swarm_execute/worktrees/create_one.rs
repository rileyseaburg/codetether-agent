//! Allocation of one task-specific swarm worktree.

use crate::swarm::SubTask;
use crate::worktree::{WorktreeInfo, WorktreeManager};
use anyhow::Context;
use std::sync::Arc;

pub(super) async fn run(
    manager: &Arc<WorktreeManager>,
    name: &str,
    task: &SubTask,
) -> anyhow::Result<Option<WorktreeInfo>> {
    if !task.needs_worktree() {
        return Ok(None);
    }
    let slug = format!("swarm_{}", uuid::Uuid::new_v4().simple());
    let worktree = manager
        .create(&slug)
        .await
        .with_context(|| format!("swarm worktree create failed for task '{name}'"))?;
    manager
        .inject_workspace_stub(&worktree.path)
        .with_context(|| format!("swarm workspace isolation setup failed for task '{name}'"))?;
    Ok(Some(worktree))
}
