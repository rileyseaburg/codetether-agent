//! Atomic replay guard for delayed verifier results and escalation decisions.
use crate::session::tasks::{GoalStatusChecked, TaskState};

pub(super) fn apply(state: &mut TaskState, update: &GoalStatusChecked) {
    if state.answer_review.is_some() {
        return;
    }
    let Some(goal) = state.goal.as_mut().filter(|goal| {
        goal.id == update.goal_id && goal.last_updated_at == update.expected_updated_at
    }) else {
        return;
    };
    goal.status = update.status;
    goal.last_updated_at = update.at;
}
