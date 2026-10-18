//! Incoming task ingestion and next-task presentation for the TUI.
//!
//! [ingest] drains the bridge receiver into [crate::WorkerState]. [take_next]
//! selects a queued task when the UI is idle and returns its display content.
mod ingest;
mod presentation;
pub use ingest::ingest;
pub use presentation::{Presentation, take_next};
