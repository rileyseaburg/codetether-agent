//! Adoption of a submitted user prompt as the active session goal.

use crate::session::tasks::{GoalRuntimeUpdate, GoalSourceKind, GoalStatus, TaskEvent};
use anyhow::Result;
use chrono::Utc;

const MAX_OBJECTIVE_CHARS: usize = 4_000;

/// Persist `prompt` as an active goal when auto-goal mode is on.
///
/// Returns `Ok(false)` without writing when the mode is off, the prompt is
/// blank, or the session already has an unfinished goal (the prompt then
/// steers that goal instead of replacing it).
///
/// # Errors
///
/// Returns an error if the session task log cannot be read or appended.
pub(crate) async fn adopt_prompt(session_id: &str, prompt: &str) -> Result<bool> {
    let objective: String = prompt.trim().chars().take(MAX_OBJECTIVE_CHARS).collect();
    if !super::auto_goal::enabled() || objective.is_empty() {
        return Ok(false);
    }
    let (log, state) = super::current(session_id).await?;
    if state.goal.is_some_and(|goal| !goal.status.is_terminal()) {
        return Ok(false);
    }
    let now = Utc::now();
    let goal_id = uuid::Uuid::new_v4().to_string();
    log.append(&TaskEvent::GoalSet {
        at: now,
        goal_id: goal_id.clone(),
        objective,
        success_criteria: Vec::new(),
        forbidden: Vec::new(),
        source_session_id: session_id.into(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    })
    .await?;
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: now,
        goal_id,
        objective: None,
        status: Some(GoalStatus::Active),
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))
    .await?;
    Ok(true)
}
