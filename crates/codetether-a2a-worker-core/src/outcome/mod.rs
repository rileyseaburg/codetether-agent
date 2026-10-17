//! Normalize claimed task execution results for completion reporting.
//!
//! [from_inner] preserves successful fields and records a failure checkpoint
//! for an execution error. [TaskInner] and [TaskOutcome] describe the boundary.
use crate::task_timeline;
mod convert;
mod failed;
mod inner;
mod record;
pub use convert::from_inner;
use failed::failed;
pub use inner::TaskInner;
pub use record::TaskOutcome;
