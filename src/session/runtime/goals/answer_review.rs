//! User-only operations for the durable goal answer-review gate.

use crate::session::tasks::{AnswerReviewAction, AnswerReviewUpdate, TaskEvent, TaskLog};
use anyhow::Result;
use chrono::Utc;
#[path = "answer_review_query.rs"]
mod query;
pub(crate) use query::{held, read, ready};

pub(crate) async fn begin(session: &str, question: &str) -> Result<bool> {
    let state = read(session)?;
    let Some(goal) = state.goal.filter(|goal| !goal.status.is_terminal()) else {
        return Ok(false);
    };
    if !goal.status.is_active() && state.answer_review.is_none() {
        return Ok(false);
    }
    record(
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

pub(crate) async fn record(
    session: &str,
    goal_id: &str,
    review_id: &str,
    decision: AnswerReviewAction,
) -> Result<()> {
    TaskLog::for_session(session)?
        .append(&TaskEvent::AnswerReview(AnswerReviewUpdate {
            at: Utc::now(),
            goal_id: goal_id.into(),
            review_id: review_id.into(),
            decision,
        }))
        .await
}

#[cfg(test)]
#[path = "answer_review_tests.rs"]
mod tests;
