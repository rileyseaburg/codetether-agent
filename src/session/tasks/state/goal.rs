//! Goal-specific state transition routing.

#[path = "goal/edit.rs"]
mod edit_goal;
#[path = "goal/runtime.rs"]
mod runtime_update;
#[path = "goal/set.rs"]
mod set_goal;

/// Replay a user edit against its captured goal revision.
pub(super) fn edit(state: &mut TaskState, edit: &crate::session::tasks::GoalEdited) {
    edit_goal::apply(state, edit);
}

use super::TaskState;
use crate::session::tasks::GoalRuntimeUpdate;
use chrono::{DateTime, Utc};

pub(super) fn set(
    state: &mut TaskState,
    at: DateTime<Utc>,
    id: &str,
    objective: &str,
    success: &[String],
    forbidden: &[String],
) {
    set_goal::apply(state, at, id, objective, success, forbidden);
}

pub(super) fn runtime(state: &mut TaskState, update: &GoalRuntimeUpdate) {
    runtime_update::apply(state, update);
}

pub(super) fn reaffirm(state: &mut TaskState, at: DateTime<Utc>) {
    if let Some(goal) = state.goal.as_mut() {
        goal.last_reaffirmed_at = at;
        goal.last_updated_at = at;
    }
}
