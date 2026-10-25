//! Record failed integrations while preserving their worktrees.

use super::SubTaskResult;

pub(crate) fn fail(result: &mut SubTaskResult, error: String) {
    tracing::warn!(subtask_id = %result.subtask_id, %error, "Swarm integration failed");
    result.success = false;
    result.error = Some(error.clone());
    result
        .result
        .push_str(&format!("\n\n--- Integration Failed ---\n{error}"));
}
