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
    let (log, now) = current::<H>(session).await?;
    let at = Utc::now();
    log.append(&TaskEvent::GoalStatusChecked(GoalStatusChecked {
        at,
        goal_id: expected.id.clone(),
        expected_updated_at: anchor(expected, now.goal.as_ref()),
        status,
    }))
    .await?;
    let (_, state) = current::<H>(session).await?;
    Ok(state.answer_review.is_none()
        && state.goal.is_some_and(|goal| {
            goal.id == expected.id && goal.last_updated_at == at && goal.status == status
        }))
}

/// Revision to compare against at replay time.
///
/// Usage accounting (tokens, elapsed time, continuations) advances `last_updated_at` while the
/// verifier runs, and the verifier's own model call is itself accounted. If only those counters
/// moved, the verified goal is substantively unchanged, so the commit anchors to the current
/// revision. Any change to identity, objective, criteria, prohibitions, status or budget keeps
/// the original revision and the commit is rejected as stale.
fn anchor(expected: &Goal, current: Option<&Goal>) -> chrono::DateTime<Utc> {
    match current {
        Some(goal) if same_substance(expected, goal) => goal.last_updated_at,
        _ => expected.last_updated_at,
    }
}

fn same_substance(a: &Goal, b: &Goal) -> bool {
    a.id == b.id
        && a.objective == b.objective
        && a.success_criteria == b.success_criteria
        && a.forbidden == b.forbidden
        && a.status == b.status
        && a.token_budget == b.token_budget
        && a.set_at == b.set_at
}

#[cfg(test)]
#[path = "transition_checked_tests.rs"]
mod tests;
