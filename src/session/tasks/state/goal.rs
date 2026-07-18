//! Goal-specific state transitions.

use super::{Goal, TaskState};
use crate::session::tasks::{GoalRuntimeUpdate, GoalStatus};
use chrono::{DateTime, Utc};

pub(super) fn set(state: &mut TaskState, at: DateTime<Utc>, id: &str,
    objective: &str, success: &[String], forbidden: &[String]) {
    let id = if id.is_empty() { format!("legacy-{}", at.timestamp_micros()) } else { id.into() };
    state.goal = Some(Goal { id, objective: objective.into(), success_criteria: success.into(),
        forbidden: forbidden.into(), status: GoalStatus::Active, token_budget: None,
        tokens_used: 0, time_used_seconds: 0, turns_used: 1, set_at: at,
        last_updated_at: at, last_reaffirmed_at: at });
}

pub(super) fn reaffirm(state: &mut TaskState, at: DateTime<Utc>) {
    if let Some(goal) = state.goal.as_mut() {
        goal.last_reaffirmed_at = at;
        goal.last_updated_at = at;
    }
}

pub(super) fn runtime(state: &mut TaskState, update: &GoalRuntimeUpdate) {
    let Some(goal) = state.goal.as_mut().filter(|goal| goal.id == update.goal_id) else { return };
    if let Some(status) = update.status { goal.status = status; }
    if let Some(budget) = update.token_budget { goal.token_budget = Some(budget); }
    goal.tokens_used = goal.tokens_used.saturating_add(update.token_delta.max(0));
    goal.time_used_seconds = goal.time_used_seconds.saturating_add(update.elapsed_seconds.max(0));
    goal.turns_used = goal.turns_used.saturating_add(update.continuation_delta);
    goal.last_updated_at = update.at;
    if goal.status.is_active() && goal.token_budget.is_some_and(|limit| goal.tokens_used >= limit) {
        goal.status = GoalStatus::BudgetLimited;
    }
}
