//! Explicit goal operations; privileged operations require the human controller.

use serde::{Deserialize, Serialize};

/// Goal operations, separate from work-item lifecycle updates.
///
/// # Examples
/// ```
/// use codetether_agent::session::tasks::GoalEditAction;
/// let action = GoalEditAction::Override;
/// assert!(matches!(action, GoalEditAction::Override));
/// ```
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalEditAction {
    /// Edit explicit fields without resetting identity or usage.
    Edit,
    /// Pause automatic continuation.
    Pause,
    /// Resume within the existing review and budget constraints.
    Resume,
    /// Clear the goal, not the session task list.
    Clear,
    /// Human-only revision override: edit, dismiss review, and reopen the goal.
    Override,
    /// Human-only completion decision; model completion still requires verification.
    #[serde(rename = "force_complete")]
    ForceComplete,
}
