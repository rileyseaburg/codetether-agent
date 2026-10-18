//! User goal controls. Native replay owns the resulting state and accounting.

mod snapshot;
use crate::tasks::{GoalEdit, TaskLog};
use codetether_goal_core::control as core;
pub use codetether_goal_core::control::GoalControlError;
use serde_json::Value;

/// Load a snapshot from native events without invoking a model.
///
/// # Errors
/// Returns storage errors without replacing them with an empty goal.
pub async fn read(session_id: &str) -> Result<Value, GoalControlError> {
    let log = TaskLog::for_session(session_id)?;
    let state = core::read(&log).await?;
    Ok(snapshot::value(session_id, &state))
}

/// Apply a user edit through the native controller and return persisted state.
///
/// # Errors
/// Returns validation, conflict, or storage errors; never retries a mutation.
pub async fn update(session_id: &str, request: GoalEdit) -> Result<Value, GoalControlError> {
    let log = TaskLog::for_session(session_id)?;
    let state = core::update(&log, request).await?;
    Ok(snapshot::value(session_id, &state))
}

/// Apply an explicit authenticated human decision, never a model-tool request.
///
/// # Errors
/// Rejects invalid input, replacement-goal identities, and storage failures.
pub async fn update_user(session_id: &str, request: GoalEdit) -> Result<Value, GoalControlError> {
    let log = TaskLog::for_session(session_id)?;
    let state = core::update_user(&log, request).await?;
    Ok(snapshot::value(session_id, &state))
}
