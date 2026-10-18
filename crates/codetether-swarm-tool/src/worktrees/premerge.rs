//! Pre-merge policy for direct swarm worktrees.

use super::state::SwarmWorktrees;
use crate::task_result::TaskResult;
use codetether_worktree::WorktreeInfo;
use codetether_worktree::swarm::branch::policy::{self, Decision};

pub(super) async fn apply(
    worktrees: &SwarmWorktrees,
    index: usize,
    result: &mut TaskResult,
    worktree: &WorktreeInfo,
    net_changes: bool,
) -> anyhow::Result<bool> {
    let decision = policy::decide(worktrees.expects_changes(index), net_changes);
    match decision {
        Decision::Merge => Ok(true),
        Decision::Cleanup => {
            result
                .output
                .push_str("\n\n--- Verification Result ---\nNo source changes to merge");
            if let Err(error) = worktrees.mgr.cleanup(&worktree.name).await {
                tracing::warn!(%error, "Failed to cleanup verification worktree");
            }
            Ok(false)
        }
        Decision::RejectMissingChanges => {
            anyhow::bail!("mutating sub-agent completed without mergeable file changes")
        }
        Decision::RejectUnexpectedChanges => {
            anyhow::bail!("non-mutating sub-agent modified source files; worktree retained")
        }
    }
}
