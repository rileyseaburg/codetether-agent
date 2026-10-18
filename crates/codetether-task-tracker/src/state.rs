//! Protocol-level active-task tracking.
//!
//! Derives the set of *currently in-flight* A2A tasks from `TaskUpdate`
//! bus messages: a task is inserted when it enters a non-terminal state and
//! removed when it reaches a terminal one. This is the source of truth for
//! "how many agents are actually working right now" — distinct from the
//! static `worker_bridge_registered_agents` roster, which only grows as peers
//! announce themselves and never reflects live activity.

use std::collections::HashMap;
use std::time::Instant;

#[path = "observe.rs"]
pub(super) mod observe;

/// In-flight task IDs, maintained from `TaskUpdate` state transitions.
///
/// # Examples
/// ```
/// let tasks = codetether_task_tracker::ActiveTasks::default();
/// assert_eq!(tasks.count(), 0);
/// ```
#[derive(Debug, Default)]
pub struct ActiveTasks {
    ids: HashMap<String, Instant>,
}

impl ActiveTasks {
    /// Number of tasks currently in a non-terminal state.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(codetether_task_tracker::ActiveTasks::default().count(), 0);
    /// ```
    pub fn count(&self) -> usize {
        self.ids.len()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
