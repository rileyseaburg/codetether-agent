//! Persist delayed lifecycle decisions without updating a replacement goal.
use super::load::current;
use crate::Journal;
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{Goal, GoalStatus, GoalStatusChecked, TaskEvent};

/// Compare at replay time, then report whether this exact update was accepted.
/// Apply a revision-checked transition and observe whether replay accepted it.
/// # Arguments
/// `session` selects the journal; `expected` is the prior goal; `status` is requested.
/// # Returns
/// Returns whether the same goal accepted this exact update without answer review.
/// # Errors
/// Journal resolution, folding or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, set_status_if_current};
/// # async fn example<H: Journal>(expected: &codetether_goal_core::Goal) -> anyhow::Result<()> {
/// let changed = set_status_if_current::<H>("session-1234", expected, codetether_goal_core::GoalStatus::Active).await?;
/// # Ok(()) }
/// ```
pub async fn set_status_if_current<H: Journal>(
    session: &str,
    expected: &Goal,
    status: GoalStatus,
) -> Result<bool> {
    let (log, _) = current::<H>(session).await?;
    let at = Utc::now();
    log.append(&TaskEvent::GoalStatusChecked(GoalStatusChecked {
        at,
        goal_id: expected.id.clone(),
        expected_updated_at: expected.last_updated_at,
        status,
    }))
    .await?;
    let (_, state) = current::<H>(session).await?;
    Ok(state.answer_review.is_none()
        && state.goal.is_some_and(|goal| {
            goal.id == expected.id && goal.last_updated_at == at && goal.status == status
        }))
}
