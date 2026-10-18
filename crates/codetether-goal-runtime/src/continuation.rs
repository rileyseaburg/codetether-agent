//! Automatic continuation construction for an idle active goal.

use super::{load::current, prompt};
use crate::Journal;
use crate::provider::{ContentPart, Message, Role};
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{GoalRuntimeUpdate, TaskEvent};

/// Build and account for an automatic continuation of the active goal.
/// # Arguments
/// `session_id` selects the goal journal.
/// # Returns
/// Returns a user-role continuation, or `None` without an active goal.
/// # Errors
/// Journal resolution, folding or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, next_message};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// let message = next_message::<H>("session-1234").await?;
/// # Ok(()) }
/// ```
pub async fn next_message<H: Journal>(session_id: &str) -> Result<Option<Message>> {
    let (log, state) = current::<H>(session_id).await?;
    let Some(goal) = state.goal.filter(|goal| goal.status.is_active()) else {
        return Ok(None);
    };
    let text = prompt::render(&goal);
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: goal.id,
        objective: None,
        status: None,
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 1,
    }))
    .await?;
    Ok(Some(Message {
        role: Role::User,
        content: vec![ContentPart::Text { text }],
    }))
}
