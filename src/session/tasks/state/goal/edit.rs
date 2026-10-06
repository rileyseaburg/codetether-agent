//! Replay one native user edit without resetting goal identity or usage.

use crate::session::tasks::{GoalEditAction, GoalEdited, GoalStatus, TaskState};

#[path = "edit_fields.rs"]
mod fields;

/// Apply only to the goal revision observed by the editing client.
pub(super) fn apply(state: &mut TaskState, edit: &GoalEdited) {
    let request = &edit.request;
    let Some(goal) = state
        .goal
        .as_mut()
        .filter(|goal| fields::matches(goal, request))
    else {
        return;
    };
    match request.action {
        GoalEditAction::Clear => {
            state.goal = None;
            state.answer_review = None;
            return;
        }
        GoalEditAction::Pause => {
            goal.status = GoalStatus::Paused;
            if let Some(review) = state.answer_review.as_mut() {
                review.resume_status = GoalStatus::Paused;
            }
        }
        GoalEditAction::Resume => {
            if state.answer_review.is_none() {
                goal.status = GoalStatus::Active;
            }
        }
        GoalEditAction::Edit => fields::apply(goal, request),
        GoalEditAction::Override | GoalEditAction::ForceComplete => {
            fields::apply(goal, request);
            state.answer_review = None;
            goal.status = if matches!(request.action, GoalEditAction::Override) {
                GoalStatus::Active
            } else {
                GoalStatus::Complete
            };
        }
    }
    fields::enforce_budget(goal);
    goal.last_updated_at = edit.at;
}
