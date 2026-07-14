//! Pre-merge handling for verification and mutation worktrees.

use super::SubTaskResult;
use crate::swarm::worktree_branch::policy::{self, Decision};
use crate::worktree::{WorktreeInfo, WorktreeManager};

pub(super) async fn before_merge(
    result: &mut SubTaskResult,
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
    expects_changes: bool,
    net_changes: bool,
) -> Result<bool, String> {
    match policy::decide(expects_changes, net_changes) {
        Decision::Merge => Ok(true),
        Decision::Cleanup => {
            result
                .result
                .push_str("\n\n--- Verification Result ---\nNo source changes to merge");
            if let Err(error) = manager.cleanup(&worktree.name).await {
                tracing::warn!(%error, "Failed to cleanup verification worktree");
            }
            Ok(false)
        }
        Decision::RejectMissingChanges => {
            Err("Mutating sub-agent completed without mergeable file changes".into())
        }
        Decision::RejectUnexpectedChanges => {
            Err("Non-mutating sub-agent modified source files; worktree retained".into())
        }
    }
}
