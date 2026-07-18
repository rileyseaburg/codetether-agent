//! Materialized session task data.

use crate::session::tasks::SessionTaskStatus;

/// Task with its latest lifecycle status.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::tasks::{SessionTaskStatus, Task};
/// let task = Task {
///     id: "t1".into(), content: "Verify tests".into(), parent_id: None,
///     status: SessionTaskStatus::Pending, last_note: None,
/// };
/// assert_eq!(task.id, "t1");
/// ```
#[derive(Clone, Debug)]
pub struct Task {
    /// Stable task identifier within the session.
    pub id: String,
    /// Human-readable unit of work.
    pub content: String,
    /// Optional parent task for nesting.
    pub parent_id: Option<String>,
    /// Latest folded lifecycle state.
    pub status: SessionTaskStatus,
    /// Optional explanation attached to the latest transition.
    pub last_note: Option<String>,
}
