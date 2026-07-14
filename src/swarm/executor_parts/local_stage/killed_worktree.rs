//! Cleanup and result annotation for a collapsed local worktree.

use super::state::State;
use crate::swarm::SubTaskResult;
use crate::worktree::WorktreeInfo;

pub(super) async fn apply(
    state: &State<'_>,
    result: &mut SubTaskResult,
    worktree: &WorktreeInfo,
    reason: &str,
) {
    result.success = false;
    result.error = Some(format!("Cancelled by collapse controller: {reason}"));
    result.result.push_str(&format!(
        "\n\n--- Collapse Controller ---\nBranch terminated: {reason}",
    ));
    if let Some(manager) = &state.manager
        && let Err(error) = manager.cleanup(&worktree.name).await
    {
        tracing::warn!(%error, "Failed to cleanup killed worktree");
    }
}
