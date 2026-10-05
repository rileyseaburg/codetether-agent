//! Persist delayed lifecycle decisions without updating a replacement goal.
use super::load::current;
use crate::session::tasks::{Goal, GoalStatus, GoalStatusChecked, TaskEvent};
use anyhow::Result;
use chrono::Utc;

/// Compare at replay time, then report whether this exact update was accepted.
pub(crate) async fn set_status_if_current(
    session: &str,
    expected: &Goal,
    status: GoalStatus,
) -> Result<bool> {
    let (log, _) = current(session).await?;
    let at = Utc::now();
    log.append(&TaskEvent::GoalStatusChecked(GoalStatusChecked {
        at,
        goal_id: expected.id.clone(),
        expected_updated_at: expected.last_updated_at,
        status,
    }))
    .await?;
    let (_, state) = current(session).await?;
    Ok(state.answer_review.is_none()
        && state.goal.is_some_and(|goal| {
            goal.id == expected.id && goal.last_updated_at == at && goal.status == status
        }))
}
