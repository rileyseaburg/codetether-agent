//! Status and lineage response contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::PersonaStatus;

/// Start/stop response status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionStatus {
    pub enabled: bool,
    pub running: bool,
    pub loop_interval_ms: u64,
    pub started_at: Option<DateTime<Utc>>,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub persona_count: usize,
    pub active_persona_count: usize,
    pub events_buffered: usize,
    pub snapshots_buffered: usize,
}

/// Result of a reap operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReapPersonaResponse {
    pub reaped_ids: Vec<String>,
    pub count: usize,
}

/// One node in the persona lineage graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    pub persona_id: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub depth: u32,
    pub status: PersonaStatus,
}

/// Full persona lineage graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageGraph {
    pub nodes: Vec<LineageNode>,
    pub roots: Vec<String>,
    pub total_edges: usize,
}
