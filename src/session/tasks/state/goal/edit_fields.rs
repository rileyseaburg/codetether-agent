//! Apply user fields and budget limits without touching accounting or tasks.

use crate::session::tasks::{Goal, GoalEdit, GoalEditAction, GoalStatus};

pub(super) fn matches(goal: &Goal, request: &GoalEdit) -> bool {
    goal.id == request.goal_id
        && (goal.last_updated_at == request.updated_at
            || matches!(
                request.action,
                GoalEditAction::Override | GoalEditAction::ForceComplete
            ))
}

pub(super) fn apply(goal: &mut Goal, request: &GoalEdit) {
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

pub(super) fn enforce_budget(goal: &mut Goal) {
    if goal.status.is_active()
        && goal
            .token_budget
            .is_some_and(|limit| goal.tokens_used >= limit)
    {
        goal.status = GoalStatus::BudgetLimited;
    }
}
