//! # Swarm Delivery Outcomes
//!
//! Delivers a completed swarm subtask's worktree changes into the checkout the
//! swarm was spawned from, as uncommitted edits. Nothing is committed to or
//! merged into the user's branch. This module prepares the worker-branch
//! commit, checks whether the branch has net changes, delegates eligibility to
//! `merge_decision`, and records the outcome on [`SubTaskResult`].
//!
//! ## Usage
//!
//! The swarm executor calls [`apply`] after a subtask finishes. Delivered and
//! already-present work is cleaned up; rejected patches keep the worktree and
//! branch and mark the result unsuccessful.

use crate::swarm::commit as worktree_commit;
use crate::{WorktreeInfo, WorktreeManager};
use codetether_swarm_core::SubTaskResult;

#[path = "decision.rs"]
mod merge_decision;
#[path = "record.rs"]
mod merge_record;

/// Delivers a completed subtask's worktree into the spawning checkout.
///
/// # Arguments
///
/// * `result` — Subtask result updated with delivery details or an integration error.
/// * `manager` — Manager for the spawning checkout; used to inspect, deliver, and clean up.
/// * `worktree` — Worktree containing the completed subtask's changes.
/// * `expects_changes` — Whether the subtask was expected to modify tracked files.
///
/// # Returns
///
/// Returns after the worktree is delivered, determined to need no delivery, or
/// fails integration. Failures are recorded in `result` rather than returned.
///
/// # Examples
///
/// A successful delivery appends a summary to the existing subtask output:
///
/// ```rust
/// let mut output = String::from("Subtask completed");
/// let summary = "Applied 2 file(s) as uncommitted changes";
/// output.push_str(&format!("\n\n--- Delivery Result ---\n{summary}"));
///
/// assert!(output.contains("--- Delivery Result ---"));
/// assert!(output.ends_with("uncommitted changes"));
/// ```
pub async fn apply(
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
    let net_changes = match crate::swarm::branch::has_net_changes(manager, worktree).await {
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
        manager.deliver(&worktree.name).await,
    )
    .await;
}
