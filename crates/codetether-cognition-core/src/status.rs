//! Status and lineage response contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::PersonaStatus;

/// Start/stop response status.
///
/// # Examples
/// ```
/// use codetether_cognition_core::status::CognitionStatus;
/// let _record = CognitionStatus { enabled: true, running: false, loop_interval_ms: 1000, started_at: None, last_tick_at: None, persona_count: 0, active_persona_count: 0, events_buffered: 0, snapshots_buffered: 0 };
/// ```
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
///
/// # Examples
/// ```
/// use codetether_cognition_core::status::ReapPersonaResponse;
/// let _record = ReapPersonaResponse { reaped_ids: vec!["p1".into()], count: 1 };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReapPersonaResponse {
    pub reaped_ids: Vec<String>,
    pub count: usize,
}

/// One node in the persona lineage graph.
///
/// # Examples
/// ```
/// use codetether_cognition_core::status::LineageNode;
/// let _record = LineageNode { persona_id: "p1".into(), parent_id: None, children: vec![], depth: 0, status: codetether_cognition_core::PersonaStatus::Active };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    pub persona_id: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub depth: u32,
    pub status: PersonaStatus,
}

/// Full persona lineage graph.
///
/// # Examples
/// ```
/// use codetether_cognition_core::status::LineageGraph;
/// let _record = LineageGraph { nodes: vec![], roots: vec![], total_edges: 0 };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageGraph {
    pub nodes: Vec<LineageNode>,
    pub roots: Vec<String>,
    pub total_edges: usize,
}
