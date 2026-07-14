//! Concurrent, fail-closed worktree provisioning for swarm subtasks.

use crate::worktree::{WorktreeInfo, WorktreeManager};
use futures::stream::{self, StreamExt};
use std::{collections::HashMap, sync::Arc};

const PARALLELISM: usize = 8;

pub(super) async fn precreate(
    manager: Arc<WorktreeManager>,
    subtask_ids: Vec<String>,
) -> HashMap<String, WorktreeInfo> {
    stream::iter(subtask_ids.into_iter().map(|id| {
        let manager = Arc::clone(&manager);
        async move { create(manager, id).await }
    }))
    .buffer_unordered(PARALLELISM)
    .filter_map(|result| async move { result })
    .collect()
    .await
}

async fn create(manager: Arc<WorktreeManager>, id: String) -> Option<(String, WorktreeInfo)> {
    let slug = id.replace('-', "_");
    let worktree = match manager.create(&slug).await {
        Ok(worktree) => worktree,
        Err(error) => {
            tracing::warn!(subtask_id = %id, %error, "Required worktree creation failed");
            return None;
        }
    };
    if let Err(error) = manager.inject_workspace_stub(&worktree.path) {
        tracing::warn!(subtask_id = %id, path = %worktree.path.display(), %error,
            "Required workspace isolation setup failed");
        return None;
    }
    tracing::info!(subtask_id = %id, path = %worktree.path.display(),
        branch = %worktree.branch, "Created isolated sub-agent worktree");
    Some((id, worktree))
}
