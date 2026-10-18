//! User-only operations for the durable goal answer-review gate.

use crate::Journal;
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{AnswerReviewAction, AnswerReviewUpdate, TaskEvent};
#[path = "answer_review_query.rs"]
mod query;
pub use query::{held, read, ready};

/// Begin user answer review for an eligible unfinished goal.
/// # Arguments
/// `session` selects the journal; `question` records the human review request.
/// # Returns
/// Returns whether a review-begin event was appended.
/// # Errors
/// Journal resolution, folding or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, answer_review};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// let started = answer_review::begin::<H>("session-1234", "Why?").await?;
/// # Ok(()) }
/// ```
pub async fn begin<H: Journal>(session: &str, question: &str) -> Result<bool> {
    let state = read::<H>(session)?;
    let Some(goal) = state.goal.filter(|goal| !goal.status.is_terminal()) else {
        return Ok(false);
    };
    if !goal.status.is_active() && state.answer_review.is_none() {
        return Ok(false);
    }
    record::<H>(
        session,
        &goal.id,
        &uuid::Uuid::new_v4().to_string(),
        AnswerReviewAction::Begin {
            question: question.into(),
        },
    )
    .await?;
    Ok(true)
}

/// Append an authenticated user answer-review decision.
/// # Arguments
/// `session`, `goal_id` and `review_id` identify the review; `decision` is the event.
/// # Returns
/// Returns unit after the event is appended.
/// # Errors
/// Journal resolution or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, answer_review};
/// # async fn example<H: Journal>(decision: codetether_goal_core::AnswerReviewAction) -> anyhow::Result<()> {
/// answer_review::record::<H>("session-1234", "goal", "review", decision).await?;
/// # Ok(()) }
/// ```
pub async fn record<H: Journal>(
    session: &str,
    goal_id: &str,
    review_id: &str,
    decision: AnswerReviewAction,
) -> Result<()> {
    H::for_session(session)?
        .append(&TaskEvent::AnswerReview(AnswerReviewUpdate {
            at: Utc::now(),
            goal_id: goal_id.into(),
            review_id: review_id.into(),
            decision,
        }))
        .await
}
