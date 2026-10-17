//! Task release lifecycle and progress cleanup.
//!
//! [release_handled_task] captures diagnostics, submits the outcome to a
//! [ReleaseTarget], records acceptance, then clears active task progress.
use anyhow::Result;
use codetether_a2a_worker_core::{outcome::TaskOutcome, task_timeline};
use std::sync::Arc;
use tokio::sync::Mutex;
mod release;
mod target;
pub use release::release_handled_task;
pub use target::ReleaseTarget;
