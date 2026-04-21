//! # Session Task Log
//!
//! Append-only JSONL event log attached to each [`Session`]. Captures the
//! session's **goal** (objective, success criteria, forbidden behaviors)
//! and the lifecycle of its **tasks** so goal-governance middleware can:
//!
//! - Inject the current objective into per-turn system prompts.
//! - Detect drift (tool calls / errors since last goal reaffirmation).
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
//! [`Session`]: super::Session

mod event;
mod log;
mod path;
mod render;
mod state;

#[allow(unused_imports)]
pub use event::{TaskEvent, TaskStatus};
pub use log::TaskLog;
pub use path::task_log_path;
pub use render::governance_block;
pub use state::TaskState;
