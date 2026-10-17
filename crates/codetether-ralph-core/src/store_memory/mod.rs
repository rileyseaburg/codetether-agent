//! In-memory run persistence using one asynchronous read/write lock.
//!
//! Construct InMemoryStore, then call its RalphStateStore methods. Operation modules
//! preserve the original locking or transport behavior for each update.
use crate::state_store::{RalphRunState, RalphRunSummary, RalphStateStore, StoryResultEntry};
use crate::{Prd, ProgressEntry, RalphStatus};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;
mod append_progress;
mod complete_run;
mod construct;
mod create_run;
mod get_run;
mod implementation;
mod list_runs;
mod record_story_result;
mod set_error;
#[cfg(test)]
mod tests;
mod update_iteration;
mod update_prd;
mod update_status;

/// In-memory state store backed by a RwLock<HashMap>
///
/// # Examples
/// ```
/// let store = codetether_ralph_core::store_memory::InMemoryStore::new();
/// let _ = store;
/// ```
pub struct InMemoryStore {
    runs: RwLock<HashMap<String, RalphRunState>>,
}
