//! Refuse model-driven goal replacement during answer review.

use crate::session::tasks::{TaskLog, TaskState};
use crate::tool::ToolResult;
use anyhow::Result;

/// Return a refusal when a persisted satisfaction hold still exists.
///
/// # Returns
///
/// `Ok(Some(error))` while held, otherwise `Ok(None)`.
///
/// # Errors
///
/// Propagates task-log read failures instead of allowing replacement.
pub(super) async fn refusal(log: &TaskLog) -> Result<Option<ToolResult>> {
    let state = TaskState::from_log(&log.read_all().await?);
    Ok(state.answer_review.is_some().then(|| {
        ToolResult::error(
            "cannot replace a goal during answer review: only the user's Yes decision \
             releases the hold; the user may explicitly discard the goal via /goal clear",
        )
    }))
}
