//! HTTP run persistence for the dashboard API.
//!
//! Construct HttpStore, then call its RalphStateStore methods. Operation modules
//! preserve the original locking or transport behavior for each update.
use crate::state_store::{RalphRunState, RalphRunSummary, RalphStateStore, StoryResultEntry};
use crate::{Prd, ProgressEntry, RalphStatus};
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tracing::{debug, warn};
mod append_progress;
mod complete_run;
mod construct;
mod create_run;
mod get_run;
mod helpers;
mod implementation;
mod list_runs;
mod record_story_result;
mod set_error;
mod update_iteration;
mod update_prd;
mod update_status;

/// HTTP-backed state store that talks to the Python A2A server
///
/// # Examples
/// ```
/// let store = codetether_ralph_core::store_http::HttpStore::new("http://localhost:8080", None);
/// let _ = store;
/// ```
pub struct HttpStore {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}
