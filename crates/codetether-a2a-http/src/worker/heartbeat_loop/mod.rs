//! Worker heartbeat scheduling, fallback submission and lease renewal.
//!
//! [start_heartbeat] starts the 30-second loop. It keeps failure accounting
//! local to that task and disables cognition briefly after fallback succeeds.
use super::{
    heartbeat_snapshot::{HeartbeatPayload, build_heartbeat_payload},
    post_heartbeat,
};
use codetether_a2a_worker_core::{
    CognitionHeartbeatConfig, HeartbeatState,
    task_config::{persistent_worker_enabled, persistent_worker_lease_seconds},
    task_timeline::TaskProgressState,
};
use reqwest::Client;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle, time::Instant};
mod extended;
mod lease;
mod run;
mod start;
mod submit;
use extended::send_extended_task_heartbeat;
pub use start::start_heartbeat;
mod configuration;
mod failures;
