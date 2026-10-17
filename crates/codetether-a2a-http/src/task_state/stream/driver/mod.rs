//! Ordered streamed-task execution over application session hooks.
//!
//! [StreamSessionHost] supplies session operations; [run] coordinates the
//! event channel, observer setup, execution, persistence and task outcome.
use super::{complete, failure, start_failure};
use codetether_a2a_core::Task;
use dashmap::DashMap;
use std::time::Instant;
use tokio::sync::mpsc;
mod host;
mod run;
pub use host::StreamSessionHost;
pub use run::run;
