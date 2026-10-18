//! Native goal storage and human-edit boundaries.
use anyhow::Result;
use codetether_goal_core::{Goal, GoalEdit, TaskLog};
use serde_json::Value;
use std::future::Future;
/// Runtime services used by explicit human goal commands.
/// # Implementors
/// Native applications resolve session journals and preserve their existing
/// user-only goal controller and auto-adoption setting.
pub trait Host {
    /// Resolve the existing journal for a session.
    /// # Errors
    /// Returns session-path or storage-resolution errors.
    fn log(session: &str) -> Result<TaskLog>;
    /// Read the current goal through the runtime's existing state reader.
    /// # Errors
    /// Returns the runtime's storage or state errors.
    fn goal(session: &str) -> Result<Option<Goal>>;
    /// Apply an explicit authenticated human edit.
    /// # Errors
    /// Returns validation, revision-conflict or persistence errors.
    fn update_user(session: &str, request: GoalEdit) -> impl Future<Output = Result<Value>> + Send;
    /// Read whether new prompts are automatically adopted as goals.
    fn auto_goal_enabled() -> bool;
    /// Change the human-selected auto-adoption setting.
    fn set_auto_goal(enabled: bool);
}
