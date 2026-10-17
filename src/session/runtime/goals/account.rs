//! Durable token and elapsed-time accounting.

use super::load::current;
use crate::session::tasks::{GoalRuntimeUpdate, TaskEvent};
use anyhow::Result;
use chrono::Utc;
use std::time::Duration;

pub(crate) async fn record_usage(session_id: &str, tokens: usize, elapsed: Duration) -> Result<()> {
    let (log, state) = current(session_id).await?;
    let Some(goal) = state.goal.filter(|goal| goal.status.is_active()) else {
        return Ok(());
    };
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: goal.id,
        objective: None,
        status: None,
        token_budget: None,
        token_delta: i64::try_from(tokens).unwrap_or(i64::MAX),
        elapsed_seconds: i64::try_from(elapsed.as_secs()).unwrap_or(i64::MAX),
        continuation_delta: 0,
    }))
    .await
}
