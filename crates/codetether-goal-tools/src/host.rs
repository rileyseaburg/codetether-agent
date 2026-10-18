//! Native goal state and revision-checked model edits.
use anyhow::Result;
use codetether_goal_core::{GoalEdit, TaskLog, TaskState, control::GoalControlError};
use serde_json::Value;
use std::future::Future;
/// Storage operations used by model-facing goal tools.
/// # Implementors
/// Native session runtimes retain journal paths, cached state and controller
/// authorization/accounting for model-originated edits.
pub trait Host {
    /// Read the current state and its append-only journal.
    /// # Errors
    /// Returns session-path, journal or runtime-state errors.
    fn current(session: &str) -> impl Future<Output = Result<(TaskLog, TaskState)>> + Send;
    /// Apply a revision-bound model edit using the existing controller.
    /// # Errors
    /// Returns validation, conflict or persistence errors.
    fn update(
        session: &str,
        edit: GoalEdit,
    ) -> impl Future<Output = Result<Value, GoalControlError>> + Send;
}
