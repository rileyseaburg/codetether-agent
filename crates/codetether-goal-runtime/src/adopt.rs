//! Adoption of a submitted user prompt as the active session goal.

use crate::Journal;
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{GoalRuntimeUpdate, GoalSourceKind, GoalStatus, TaskEvent};

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
/// Adopt a user prompt only when auto-goal mode permits a new goal.
/// # Arguments
/// `session_id` selects the journal; `prompt` supplies the user objective.
/// # Returns
/// Returns whether a new goal was persisted; unfinished goals are preserved.
/// # Errors
/// Journal resolution, folding or event append fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, adopt_prompt};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// let created = adopt_prompt::<H>("session-1234", "Review the project").await?;
/// # Ok(()) }
/// ```
pub async fn adopt_prompt<H: Journal>(session_id: &str, prompt: &str) -> Result<bool> {
    let objective: String = prompt.trim().chars().take(MAX_OBJECTIVE_CHARS).collect();
    if !super::auto_goal::enabled() || objective.is_empty() {
        return Ok(false);
    }
    let (log, state) = super::current::<H>(session_id).await?;
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
