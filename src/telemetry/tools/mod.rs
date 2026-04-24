//! # Tool telemetry
//!
//! Per-tool execution tracking, including success/failure counts, duration,
//! and the file changes each tool performed.
//!
//! | File | Responsibility |
//! |---|---|
//! | [`counter`]     | [`AtomicToolCounter`] — lock-free success/failure counter |
//! | [`file_change`] | [`FileChange`] records emitted by filesystem tools |
//! | [`execution`]   | [`ToolExecution`] — a single tool invocation record |

pub mod counter;
pub mod execution;
pub mod file_change;

pub use counter::AtomicToolCounter;
pub use execution::ToolExecution;
pub use file_change::FileChange;
