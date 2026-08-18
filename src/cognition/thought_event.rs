//! Streamable thought event and distilled memory snapshot contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ThoughtEventType;

/// Streamable thought event contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEvent {
    pub id: String,
    pub event_type: ThoughtEventType,
    pub persona_id: Option<String>,
    pub swarm_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub payload: serde_json::Value,
}

/// Distilled memory snapshot contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub id: String,
    pub generated_at: DateTime<Utc>,
    pub swarm_id: Option<String>,
    pub persona_scope: Vec<String>,
    pub summary: String,
    pub hot_event_count: usize,
    pub warm_fact_count: usize,
    pub cold_snapshot_count: usize,
    pub metadata: HashMap<String, serde_json::Value>,
}
