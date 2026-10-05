//! User goal controls. Native replay owns the resulting state and accounting.

mod edit;
mod error;
mod input;
mod snapshot;
mod validate;

use crate::session::tasks::{GoalEdit, TaskLog, TaskState};
pub(crate) use error::GoalControlError;
use serde_json::Value;

/// Load a snapshot from native events without invoking a model.
///
/// # Errors
/// Returns storage errors without replacing them with an empty goal.
pub(crate) async fn read(session_id: &str) -> Result<Value, GoalControlError> {
    let log = TaskLog::for_session(session_id)?;
    let state = TaskState::from_log(&log.read_all().await?);
    Ok(snapshot::value(session_id, &state))
}

/// Apply a user edit through the native controller and return persisted state.
///
/// # Errors
/// Returns validation, conflict, or storage errors; never retries a mutation.
pub(crate) async fn update(session_id: &str, request: GoalEdit) -> Result<Value, GoalControlError> {
    let log = TaskLog::for_session(session_id)?;
    let state = edit::apply(&log, request).await?;
    Ok(snapshot::value(session_id, &state))
}

#[cfg(test)]
mod tests;
