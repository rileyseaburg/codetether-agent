//! Task pipeline instrumentation for debug tracing and timeout diagnosis.
//!
//! [TaskTimeline] records [TaskCheckpoint] entries and produces diagnostics.
//! [TaskProgressState] exposes progress to heartbeat consumers.
//!
//! ```rust,no_run
//! use codetether_a2a_worker_core::task_timeline::{TaskTimeline, TaskCheckpoint};
//! let mut timeline = TaskTimeline::new("task-1", 1200);
//! timeline.checkpoint(TaskCheckpoint::TaskReceived);
//! assert!(timeline.reached(TaskCheckpoint::TaskReceived));
//! ```

mod checkpoint;
mod checkpoint_all;
mod checkpoint_display;
mod diagnostics;
mod diagnostics_emit;
mod diagnostics_json;
mod entry;
mod progress;
mod timeline;
mod timeline_helpers;

pub use checkpoint::TaskCheckpoint;
pub use entry::CheckpointEntry;
pub use progress::TaskProgressState;
pub use timeline::TaskTimeline;

mod checkpoint_name;

mod timeline_checkpoints;

mod timeline_time;
