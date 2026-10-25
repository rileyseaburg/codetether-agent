//! Record a swarm worktree delivery without hiding failed artifacts.

use super::SubTaskResult;
use crate::{Delivery, WorktreeInfo, WorktreeManager};

#[path = "failure.rs"]
mod failure;
pub(super) use failure::fail;

pub(super) async fn apply(
    result: &mut SubTaskResult,
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
    delivered: anyhow::Result<Delivery>,
) {
    match delivered {
        Ok(delivery) if delivery.status.is_delivered() => {
            success(result, manager, worktree, delivery).await
        }
        Ok(delivery) => fail(result, delivery.summary()),
        Err(error) => fail(result, format!("Failed to deliver worktree: {error}")),
    }
}

async fn success(
    result: &mut SubTaskResult,
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
    delivery: Delivery,
) {
    tracing::info!(subtask_id = %result.subtask_id, files = delivery.files.len(),
        status = delivery.status.as_str(), "Delivered swarm worktree changes");
    result.result.push_str(&format!(
        "\n\n--- Delivery Result ---\n{}",
        delivery.summary()
    ));
    if let Err(error) = manager.discard_delivered(&worktree.name).await {
        tracing::warn!(%error, "Failed to cleanup delivered swarm worktree");
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
