//! Append-only goal and task governance, independent of model and UI runtimes.
//!
//! [`TaskLog`] reads and appends JSONL events at an explicit path;
//! [`TaskState`] folds those events without changing their persistence format.
//! [`control`] provides revision-checked model and authenticated-human edits.
//!
//! # Examples
//! ```
//! use codetether_goal_core::TaskState;
//! assert!(TaskState::from_log(&[]).goal.is_none());
//! ```

pub mod control;
mod event;
mod log;
mod render;
mod state;

pub use event::{
    AnswerReviewAction, AnswerReviewUpdate, DriftDetected, GoalEdit, GoalEditAction, GoalEdited,
    GoalReaffirmation, GoalRuntimeUpdate, GoalSourceKind, GoalStatus, GoalStatusChecked,
    SessionTaskStatus, TaskEvent,
};
pub use log::TaskLog;
pub use render::{goal_block, governance_block, task_block};
pub use state::{AnswerReview, Goal, Task, TaskState};

#[cfg(test)]
pub(crate) use state::answer_review_test_support;
