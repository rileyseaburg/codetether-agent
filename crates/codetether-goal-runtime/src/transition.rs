//! Goal lifecycle transitions initiated by tools or runtime failures.

use super::load::current;
use crate::Journal;
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{GoalRuntimeUpdate, GoalStatus, TaskEvent};

/// Append a goal status transition unless answer review holds the goal.
/// # Arguments
/// `session_id` selects the journal; `status` is the requested transition.
/// # Returns
/// Returns whether a goal transition was appended.
/// # Errors
/// Journal resolution, folding or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, set_status};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// let changed = set_status::<H>("session-1234", codetether_goal_core::GoalStatus::Active).await?;
/// # Ok(()) }
/// ```
pub async fn set_status<H: Journal>(session_id: &str, status: GoalStatus) -> Result<bool> {
    let (log, state) = current::<H>(session_id).await?;
    if state.answer_review.is_some() {
        return Ok(false);
    }
    let Some(goal) = state.goal else {
        return Ok(false);
    };
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: goal.id,
        objective: None,
        status: Some(status),
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))
    .await?;
    Ok(true)
}
