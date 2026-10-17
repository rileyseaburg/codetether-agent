//! Deserialized cognition service snapshots.

use serde::Deserialize;
use std::collections::HashMap;

/// Deserialized status of the remote cognition loop.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::{CognitionStatusSnapshot};
/// let status: CognitionStatusSnapshot = serde_json::from_str(r#"{"running":true}"#).unwrap();
/// assert!(status.running);
/// assert_eq!(status.active_persona_count, 0);
/// ```
#[derive(Debug, Deserialize)]
pub struct CognitionStatusSnapshot {
    pub running: bool,
    #[serde(default)]
    pub last_tick_at: Option<String>,
    #[serde(default)]
    pub active_persona_count: usize,
    #[serde(default)]
    pub events_buffered: usize,
    #[serde(default)]
    pub snapshots_buffered: usize,
    #[serde(default)]
    pub loop_interval_ms: u64,
}

/// Latest cognition summary and its extension metadata.
///
/// # Examples
/// ```
/// use codetether_a2a_worker_core::{CognitionLatestSnapshot};
/// let value: CognitionLatestSnapshot = serde_json::from_str(r#"{"generated_at":"now","summary":"ready"}"#).unwrap();
/// assert!(value.metadata.is_empty());
/// ```
#[derive(Debug, Deserialize)]
pub struct CognitionLatestSnapshot {
    pub generated_at: String,
    pub summary: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
