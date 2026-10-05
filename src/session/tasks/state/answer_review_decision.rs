//! Fold only decisions for the current question and goal identities.

use super::TaskState;
use crate::session::tasks::{AnswerReviewAction as Action, AnswerReviewUpdate};

pub(super) fn apply(state: &mut TaskState, update: &AnswerReviewUpdate) {
    let Some(goal) = state.goal.as_mut() else {
        return;
    };
    let Some(review) = state
        .answer_review
        .as_mut()
        .filter(|review| review.id == update.review_id && review.goal_id == update.goal_id)
    else {
        return;
    };
    match update.decision {
        Action::Begin { .. } => {}
        Action::Answered => {
            if review.question.take().is_some() {
                review.ready = true;
            }
        }
        Action::Unsatisfied => {
            review.question = None;
            review.ready = false;
        }
        Action::Satisfied if review.ready => {
            goal.status = super::answer_review_resume::status(goal, review.resume_status);
            goal.last_updated_at = update.at;
            state.answer_review = None;
        }
        Action::Satisfied => {}
    }
}
