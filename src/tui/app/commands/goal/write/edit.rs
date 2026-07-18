//! `/goal edit` persistence.

use crate::session::tasks::{GoalRuntimeUpdate, TaskEvent, TaskLog, TaskState};
use anyhow::{Result, anyhow};
use chrono::Utc;

pub(super) async fn run(session_id: &str, objective: &str) -> Result<String> {
    if objective.is_empty() {
        return Err(anyhow!("usage: /goal edit <objective>"));
    }
    let log = TaskLog::for_session(session_id)?;
    let state = TaskState::from_log(&log.read_all().await?);
    let goal = state.goal.ok_or_else(|| anyhow!("no goal exists"))?;
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: goal.id,
        objective: Some(objective.into()),
        status: None,
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))
    .await?;
    Ok(format!("Goal edited: {objective}"))
}
