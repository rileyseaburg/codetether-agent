//! Materialization of a newly set goal.

use super::super::{Goal, TaskState};
use crate::session::tasks::GoalStatus;
use chrono::{DateTime, Utc};

pub(super) fn apply(
    state: &mut TaskState,
    at: DateTime<Utc>,
    id: &str,
    objective: &str,
    success: &[String],
    forbidden: &[String],
) {
    let id = if id.is_empty() {
        format!("legacy-{}", at.timestamp_micros())
    } else {
        id.into()
    };
    state.goal = Some(Goal {
        id,
        objective: objective.into(),
        success_criteria: success.into(),
        forbidden: forbidden.into(),
        status: GoalStatus::Active,
        token_budget: None,
        tokens_used: 0,
        time_used_seconds: 0,
        turns_used: 1,
        set_at: at,
        last_updated_at: at,
        last_reaffirmed_at: at,
    });
}
