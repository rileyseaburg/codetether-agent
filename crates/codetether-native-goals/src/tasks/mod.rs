//! # Session Task Log
//!
//! Append-only JSONL event log attached to each session. Captures the
//! session's **goal** (objective, success criteria, forbidden behaviors)
//! and the lifecycle of its **tasks** so goal-governance middleware can:
//!
//! - Inject the current objective into per-turn system prompts.
//! - Account token/time progress and continue active goals automatically.
//! - Survive crashes and session resumes without losing intent.
//!
//! ## File layout
//!
//! Each session gets a sibling file next to its `.json`:
//!
//! ```text
//! <data_dir>/sessions/<session-id>.json        // messages
//! <data_dir>/sessions/<session-id>.tasks.jsonl // this log
//! ```
//!
//! ## Event shape
//!
//! Events are JSON objects, one per line, tagged by `kind`. See
//! [`TaskEvent`]. The log is append-only; mutations are expressed as new
//! events (e.g. [`TaskEvent::TaskStatus`] with `status = done`) rather
//! than in-place edits. [`TaskState::from_log`] folds the stream into
//! the current goal + open task list.
//!
//! Session paths use the shared durable session store.

pub mod control;
pub use codetether_goal_core as event;
mod log;
mod path;
pub use codetether_goal_core as render;
pub use codetether_goal_core as state;
pub use event::{GoalEdit, GoalEditAction, GoalEdited, GoalStatusChecked};
pub mod state_cache;
pub use codetether_goal_runtime::turn_instructions;

#[allow(unused_imports)]
pub use event::{
    AnswerReviewAction, GoalRuntimeUpdate, GoalSourceKind, GoalStatus, SessionTaskStatus, TaskEvent,
};
pub use event::{AnswerReviewUpdate, GoalReaffirmation};
pub use log::TaskLog;
pub use path::task_log_path;
pub use render::{goal_block, governance_block, task_block};
pub use state::{AnswerReview, Goal, Task, TaskState};
