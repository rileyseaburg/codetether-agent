//! Revision-checked edits through an append-only goal journal.
//!
//! [`update`] accepts model-originated changes; [`update_user`] is reserved for
//! an authenticated human boundary. Neither operation retries a mutation.

mod accepted;
mod edit;
mod error;
mod input;
mod user_validate;
mod validate;

#[cfg(test)]
mod tests;

use crate::{GoalEdit, TaskLog, TaskState};
pub use error::GoalControlError;

/// Read persisted state without invoking a model.
///
/// # Arguments
/// * `log` — Journal to replay.
/// # Returns
/// The materialized goal and tasks.
/// # Errors
/// Returns storage errors rather than an empty snapshot.
pub async fn read(log: &TaskLog) -> Result<TaskState, GoalControlError> {
    Ok(TaskState::from_log(&log.read_all().await?))
}

/// Apply a model-originated edit, rejecting budget and privileged controls.
///
/// # Arguments
/// * `log` — Journal to append to.
/// * `request` — Revision-checked edit.
/// # Returns
/// Persisted state after an accepted edit.
/// # Errors
/// Returns validation, revision conflict, or storage failures.
pub async fn update(log: &TaskLog, request: GoalEdit) -> Result<TaskState, GoalControlError> {
    edit::apply(log, request).await
}

/// Apply an explicit authenticated human edit, never a model-tool request.
///
/// # Arguments
/// * `log` — Journal to append to.
/// * `request` — Human-authorized revision-checked edit.
/// # Returns
/// Persisted state after an accepted edit.
/// # Errors
/// Returns invalid input, replacement-goal conflicts, or storage failures.
pub async fn update_user(log: &TaskLog, request: GoalEdit) -> Result<TaskState, GoalControlError> {
    edit::apply_user(log, request).await
}
