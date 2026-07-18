//! Goal lifecycle transitions initiated by tools or runtime failures.

use super::load::current;
use crate::session::tasks::{GoalRuntimeUpdate, GoalStatus, TaskEvent};
use anyhow::Result;
use chrono::Utc;

pub(crate) async fn set_status(session_id: &str, status: GoalStatus) -> Result<bool> {
    let (log, state) = current(session_id).await?;
    let Some(goal) = state.goal else { return Ok(false) };
    log.append(&TaskEvent::GoalRuntime { update: GoalRuntimeUpdate {
        at: Utc::now(), goal_id: goal.id, status: Some(status), token_budget: None,
        token_delta: 0, elapsed_seconds: 0, continuation_delta: 0,
    }}).await?;
    Ok(true)
}

pub(crate) async fn block_after_error(session_id: &str) {
    let Ok((_, state)) = current(session_id).await else { return };
    if state.goal.as_ref().is_some_and(|goal| goal.status.is_active())
        && let Err(error) = set_status(session_id, GoalStatus::Blocked).await {
        tracing::warn!(error = %error, session_id, "Failed to block goal after turn error");
    }
}
