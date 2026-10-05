//! Model-facing goal creation and replacement event handler.

use super::super::params::Params;
use crate::session::tasks::{TaskEvent, TaskLog};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use chrono::Utc;

/// Validate replacement eligibility, then append the model's goal event.
pub(crate) async fn set_goal(log: &TaskLog, p: Params) -> Result<ToolResult> {
    if let Some(refused) = super::set_guard::refusal(log).await? {
        return Ok(refused);
    }
    let objective = p
        .objective
        .ok_or_else(|| anyhow!("`objective` is required"))?;
    log.append(&TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: uuid::Uuid::new_v4().to_string(),
        objective: objective.clone(),
        success_criteria: p.success_criteria.unwrap_or_default(),
        forbidden: p.forbidden.unwrap_or_default(),
        source_session_id: String::new(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: Default::default(),
        confidence: 0.0,
    })
    .await?;
    Ok(ToolResult::success(format!("Goal set: {objective}")))
}
