//! Replay one native user edit without resetting goal identity or usage.

use crate::session::tasks::{GoalEditAction, GoalEdited, GoalStatus, TaskState};

/// Apply only to the goal revision observed by the editing client.
pub(super) fn apply(state: &mut TaskState, edit: &GoalEdited) {
    let request = &edit.request;
    let Some(goal) = state
        .goal
        .as_mut()
        .filter(|goal| goal.id == request.goal_id && goal.last_updated_at == request.updated_at)
    else {
        return;
    };
    match request.action {
        GoalEditAction::Clear => {
            state.goal = None;
            state.answer_review = None;
            return;
        }
        GoalEditAction::Pause => goal.status = GoalStatus::Paused,
        GoalEditAction::Resume => {
            if state.answer_review.is_none() {
                goal.status = GoalStatus::Active;
            }
        }
        GoalEditAction::Edit => {
            if let Some(value) = &request.objective {
                goal.objective = value.clone();
            }
            if let Some(value) = &request.success_criteria {
                goal.success_criteria = value.clone();
            }
            if let Some(value) = &request.forbidden {
                goal.forbidden = value.clone();
            }
            if let Some(value) = request.token_budget {
                goal.token_budget = value;
            }
        }
    }
    if goal.status.is_active()
        && goal
            .token_budget
            .is_some_and(|limit| goal.tokens_used >= limit)
    {
        goal.status = GoalStatus::BudgetLimited;
    }
    goal.last_updated_at = edit.at;
}
