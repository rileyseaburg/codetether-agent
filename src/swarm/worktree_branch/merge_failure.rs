//! Record failed integrations while preserving their worktrees.

use super::SubTaskResult;
use crate::worktree::MergeResult;

pub(super) fn merge(result: &mut SubTaskResult, merged: &MergeResult) {
    let kind = if merged.aborted {
        "aborted"
    } else {
        "conflicted"
    };
    fail(result, format!("Worktree merge {kind}: {}", merged.summary));
}

pub(crate) fn fail(result: &mut SubTaskResult, error: String) {
    tracing::warn!(subtask_id = %result.subtask_id, %error, "Swarm integration failed");
    result.success = false;
    result.error = Some(error.clone());
    result
        .result
        .push_str(&format!("\n\n--- Integration Failed ---\n{error}"));
}
