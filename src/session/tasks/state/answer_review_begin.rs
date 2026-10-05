//! Begin or replace a question without losing the original resume status.

use super::TaskState;
use crate::session::tasks::{AnswerReview, AnswerReviewUpdate, GoalStatus};

pub(super) fn apply(state: &mut TaskState, update: &AnswerReviewUpdate, question: &str) {
    let Some(goal) = state
        .goal
        .as_mut()
        .filter(|goal| !goal.status.is_terminal())
    else {
        return;
    };
    let resume_status = state
        .answer_review
        .as_ref()
        .map_or(goal.status, |review| review.resume_status);
    state.answer_review = Some(AnswerReview {
        id: update.review_id.clone(),
        goal_id: update.goal_id.clone(),
        question: Some(question.into()),
        ready: false,
        resume_status,
    });
    goal.status = GoalStatus::Paused;
    goal.last_updated_at = update.at;
}
