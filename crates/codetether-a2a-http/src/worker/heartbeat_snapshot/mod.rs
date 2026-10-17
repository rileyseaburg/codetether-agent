//! Snapshot worker state and progress for heartbeat submission.
//!
//! [build_heartbeat_payload] updates status before collecting state, optionally
//! attaches cognition, and returns [HeartbeatPayload] with a base fallback.
use super::cognition::fetch_cognition_heartbeat_payload;
use codetether_a2a_worker_core::{
    CognitionHeartbeatConfig, HeartbeatState, WorkerStatus, task_timeline::TaskProgressState,
};
use reqwest::Client;
use std::sync::Arc;
use tokio::{sync::Mutex, time::Instant};
mod build;
mod progress;
mod record;
mod state;
pub use build::build_heartbeat_payload;
pub use record::HeartbeatPayload;
