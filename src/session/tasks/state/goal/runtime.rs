//! Materialization of goal runtime updates.

use super::super::TaskState;
use crate::session::tasks::{GoalRuntimeUpdate, GoalStatus};

pub(super) fn apply(state: &mut TaskState, update: &GoalRuntimeUpdate) {
    let Some(goal) = state.goal.as_mut().filter(|goal| goal.id == update.goal_id) else {
        return;
    };
    if let Some(objective) = &update.objective {
        goal.objective = objective.clone();
    }
    if let Some(status) = update.status {
        goal.status = status;
    }
    if let Some(budget) = update.token_budget {
        goal.token_budget = Some(budget);
    }
    goal.tokens_used = goal.tokens_used.saturating_add(update.token_delta.max(0));
    goal.time_used_seconds = goal
        .time_used_seconds
        .saturating_add(update.elapsed_seconds.max(0));
    goal.turns_used = goal.turns_used.saturating_add(update.continuation_delta);
    goal.last_updated_at = update.at;
    if goal.status.is_active()
        && goal
            .token_budget
            .is_some_and(|limit| goal.tokens_used >= limit)
    {
        goal.status = GoalStatus::BudgetLimited;
    }
}
