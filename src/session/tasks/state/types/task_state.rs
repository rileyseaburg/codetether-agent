//! Folded goal and task state.

use super::{Goal, Task};
use std::collections::BTreeMap;

/// Folded view reconstructed by replaying the append-only session task log.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::tasks::TaskState;
/// let state = TaskState::default();
/// assert!(state.goal.is_none());
/// assert!(state.tasks.is_empty());
/// ```
#[derive(Clone, Debug, Default)]
pub struct TaskState {
    /// Current goal, if one has been set and not cleared.
    pub goal: Option<Goal>,
    /// Tasks indexed by stable task identifier.
    pub tasks: BTreeMap<String, Task>,
}
