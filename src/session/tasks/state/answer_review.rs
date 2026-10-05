//! Fold answer-review decisions without letting runtime updates release them.

use super::TaskState;
use crate::session::tasks::{AnswerReviewAction, AnswerReviewUpdate};

pub(super) fn apply(state: &mut TaskState, update: &AnswerReviewUpdate) {
    if state
        .goal
        .as_ref()
        .is_none_or(|goal| goal.id != update.goal_id)
    {
        return;
    }
    if let AnswerReviewAction::Begin { question } = &update.decision {
        super::answer_review_begin::apply(state, update, question);
    } else {
        super::answer_review_decision::apply(state, update);
    }
}

#[cfg(test)]
#[path = "answer_review_tests.rs"]
mod tests;
