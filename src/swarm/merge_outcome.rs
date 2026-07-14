//! # Swarm Merge Outcomes
//!
//! Applies a completed swarm subtask's worktree changes to the parent branch.
//! This module prepares a commit, checks whether the branch has net changes,
//! delegates merge eligibility to `merge_decision`, and records the resulting
//! success or failure on [`SubTaskResult`].
//!
//! ## Usage
//!
//! The swarm executor calls [`apply`] after a subtask finishes. Callers provide
//! the subtask result and its [`WorktreeInfo`], plus whether changes and
//! verification output are expected. Successful merges are appended to the
//! result and cleaned up; integration failures mark the result unsuccessful.

use super::SubTaskResult;
use crate::swarm::worktree_commit;
use crate::worktree::{WorktreeInfo, WorktreeManager};

#[path = "merge_decision.rs"]
mod merge_decision;
#[path = "worktree_branch/merge_record.rs"]
mod merge_record;

/// Integrates a completed subtask's worktree into the parent branch.
///
/// # Arguments
///
/// * `result` — Subtask result updated with merge details or an integration error.
/// * `manager` — Manager used to inspect, merge, and clean up the worktree.
/// * `worktree` — Worktree containing the completed subtask's changes.
/// * `expects_changes` — Whether the subtask was expected to modify tracked files.
///
/// # Returns
///
/// Returns after the worktree is merged, determined to need no merge, or fails
/// integration. Failures are recorded in `result` rather than returned.
///
/// # Examples
///
/// A successful integration appends a merge summary to the existing subtask
/// output:
///
/// ```rust
/// let mut output = String::from("Subtask completed");
/// let summary = "Merged 2 files";
/// output.push_str(&format!("\n\n--- Merge Result ---\n{summary}"));
///
/// assert!(output.contains("--- Merge Result ---"));
/// assert!(output.ends_with("Merged 2 files"));
/// ```
pub(super) async fn apply(
    result: &mut SubTaskResult,
    manager: &WorktreeManager,
    worktree: &WorktreeInfo,
    expects_changes: bool,
) {
    if let Err(error) = worktree_commit::prepare(worktree, &result.subtask_id).await {
        return merge_record::fail(
            result,
            format!("Failed to commit worktree changes: {error}"),
        );
    }
    let net_changes = match crate::swarm::worktree_branch::has_net_changes(manager, worktree).await
    {
        Ok(net_changes) => net_changes,
        Err(error) => {
            return merge_record::fail(
                result,
                format!("Failed to inspect worktree branch: {error}"),
            );
        }
    };
    match merge_decision::before_merge(result, manager, worktree, expects_changes, net_changes)
        .await
    {
        Ok(true) => {}
        Ok(false) => return,
        Err(error) => return merge_record::fail(result, error),
    }
    merge_record::apply(
        result,
        manager,
        worktree,
        manager.merge(&worktree.name).await,
    )
    .await;
}
