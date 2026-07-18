//! Automatic continuation construction for an idle active goal.

use super::{load::current, prompt};
use crate::provider::{ContentPart, Message, Role};
use crate::session::tasks::{GoalRuntimeUpdate, TaskEvent};
use anyhow::Result;
use chrono::Utc;

pub(crate) async fn next_message(session_id: &str) -> Result<Option<Message>> {
    let (log, state) = current(session_id).await?;
    let Some(goal) = state.goal.filter(|goal| goal.status.is_active()) else { return Ok(None) };
    let text = prompt::render(&goal);
    log.append(&TaskEvent::GoalRuntime { update: GoalRuntimeUpdate {
        at: Utc::now(), goal_id: goal.id, status: None, token_budget: None,
        token_delta: 0, elapsed_seconds: 0, continuation_delta: 1,
    }}).await?;
    Ok(Some(Message { role: Role::User, content: vec![ContentPart::Text { text }] }))
}
