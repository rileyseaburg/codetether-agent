//! Materialized goal and task state reconstructed from the append-only log.

mod apply;
mod goal;
mod task;
mod types;

pub use types::{Goal, Task, TaskState};

use super::TaskEvent;

impl TaskState {
    /// Fold events in insertion order into the current state.
    pub fn from_log(events: &[TaskEvent]) -> Self {
        let mut state = Self::default();
        events.iter().for_each(|event| state.apply(event));
        state
    }

    /// Returns tasks that are pending or in progress, ordered by id.
    pub fn open_tasks(&self) -> Vec<&Task> {
        self.tasks.values().filter(|task| task.status.is_open()).collect()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
