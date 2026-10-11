//! Materialized goal and task state reconstructed from the append-only log.

mod apply;
mod goal;
mod task;
mod types;

pub use types::{AnswerReview, Goal, Task, TaskState};
mod answer_review;
mod answer_review_begin;
mod answer_review_decision;
mod answer_review_resume;

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
        self.tasks
            .values()
            .filter(|task| task.status.is_open())
            .collect()
    }
}

#[cfg(test)]
pub(crate) mod answer_review_test_support;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
