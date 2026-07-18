//! Goal lifecycle transitions initiated by tools or runtime failures.

use super::load::current;
use crate::session::tasks::{GoalRuntimeUpdate, GoalStatus, TaskEvent};
use anyhow::Result;
use chrono::Utc;

pub(crate) async fn set_status(session_id: &str, status: GoalStatus) -> Result<bool> {
    let (log, state) = current(session_id).await?;
    let Some(goal) = state.goal else {
        return Ok(false);
    };
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: goal.id,
        objective: None,
        status: Some(status),
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))
    .await?;
    Ok(true)
}
