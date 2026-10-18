//! Durable token and elapsed-time accounting.

use super::load::current;
use crate::Journal;
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{GoalRuntimeUpdate, TaskEvent};
use std::time::Duration;

/// Append token and elapsed-time usage to the current active goal.
/// # Arguments
/// `session_id` selects the journal; `tokens` and `elapsed` are this increment.
/// # Returns
/// Returns unit; sessions without an active goal are left unchanged.
/// # Errors
/// Journal resolution, folding or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, record_usage};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// record_usage::<H>("session-1234", 10, std::time::Duration::from_secs(1)).await?;
/// # Ok(()) }
/// ```
pub async fn record_usage<H: Journal>(
    session_id: &str,
    tokens: usize,
    elapsed: Duration,
) -> Result<()> {
    let (log, state) = current::<H>(session_id).await?;
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
