//! HTTP bus ingestion and control-plane endpoint discovery.
//!
//! [BusBridge] buffers events from an authenticated SSE stream; construct it
//! with [BusBridge::with_auth], then call [BusBridge::spawn] for ingestion.
//! The resolver functions discover worker endpoints from worker IDs, workspace
//! assignments, local paths or an unambiguous active-worker selection.
use chrono::{DateTime, Utc};
use codetether_bus_core::BusEnvelope;
use serde::Deserialize;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::sync::RwLock;

/// Maximum number of envelopes kept in the ring buffer.
const DEFAULT_BUFFER_SIZE: usize = 1_000;

/// Reconnect delay after a transient SSE failure.
const RECONNECT_DELAY: std::time::Duration = std::time::Duration::from_secs(3);
mod buffer;
mod construct;
mod default_worker;
mod parse;
mod path_match;
mod path_normalize;
mod path_suffix;
mod query;
mod reconnect;
mod spawn;
mod status;
mod stream;
mod topic;
mod worker;
mod workspace;
mod workspace_path;
mod workspace_types;
use path_match::best_workspace_match;
use path_normalize::normalize_local_path;
use path_suffix::shared_path_suffix_score;
use topic::topic_matches;
use workspace_types::{WorkspaceDetails, WorkspaceSummary};
mod types;
pub use default_worker::resolve_default_worker_bus_url;
pub use types::{BusBridge, BusBridgeStatus};
pub use worker::resolve_worker_bus_url;
pub use workspace::resolve_worker_bus_url_for_workspace;
pub use workspace_path::resolve_workspace_id_from_path;
#[cfg(test)]
mod tests;

mod active_workers;
use active_workers::active_workers;
