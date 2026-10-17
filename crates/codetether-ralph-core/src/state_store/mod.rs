//! Persistent Ralph run state and storage contract.
//!
//! Implement RalphStateStore to store RalphRunState records and their updates.
//! RalphRunSummary supports listings; StoryResultEntry records one story attempt.
use crate::{Prd, ProgressEntry, RalphConfig, RalphStatus};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
mod ralph_run_state;
mod ralph_run_summary;
mod store_trait;
mod story_result_entry;
pub use ralph_run_state::RalphRunState;
pub use ralph_run_summary::RalphRunSummary;
pub use store_trait::RalphStateStore;
pub use story_result_entry::StoryResultEntry;

/// Scheduled initial, iteration, story and terminal run updates.
pub mod updates;
