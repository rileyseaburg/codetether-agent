//! Native goal controller boundary for session HTTP routes.
use codetether_goal_core::{GoalEdit, control::GoalControlError};
use serde_json::Value;
/// Extend session loading with the existing native human goal controls.
/// # Implementors
/// Applications bind their durable goal controller to this interface.
#[async_trait::async_trait]
pub trait GoalHost: crate::SessionHost {
    /// Read the persisted native goal projection.
    /// # Errors
    /// Returns typed storage failures without synthesizing an empty goal.
    async fn read_goal(id: &str) -> Result<Value, GoalControlError>;
    /// Apply a human-authorized edit with revision and lifecycle validation.
    /// # Errors
    /// Returns validation, conflict or storage failures without retrying.
    async fn update_goal(id: &str, edit: GoalEdit) -> Result<Value, GoalControlError>;
}
