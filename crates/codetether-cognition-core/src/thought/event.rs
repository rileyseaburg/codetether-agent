//! Streamable thought event and distilled memory snapshot contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ThoughtEventType;

/// Streamable thought event contract.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = ThoughtEvent { id: "e1".into(), event_type: ThoughtEventType::ThoughtGenerated, persona_id: None, swarm_id: None, timestamp: now, payload: serde_json::json!({}) };
/// assert_eq!(value.event_type, ThoughtEventType::ThoughtGenerated);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEvent {
    /// Stable record identifier.
    pub id: String,
    /// Kind of cognition event.
    pub event_type: ThoughtEventType,
    /// Persona associated with this record or request.
    pub persona_id: Option<String>,
    /// Optional owning swarm identifier.
    pub swarm_id: Option<String>,
    /// Time the event occurred.
    pub timestamp: DateTime<Utc>,
    /// Event-specific structured data.
    pub payload: serde_json::Value,
}

/// Distilled memory snapshot contract.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = MemorySnapshot { id: "s1".into(), generated_at: now, swarm_id: None, persona_scope: vec![], summary: "No changes".into(), hot_event_count: 0, warm_fact_count: 0, cold_snapshot_count: 0, metadata: Default::default() };
/// assert!(value.persona_scope.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Stable record identifier.
    pub id: String,
    /// Snapshot creation time.
    pub generated_at: DateTime<Utc>,
    /// Optional owning swarm identifier.
    pub swarm_id: Option<String>,
    /// Personas covered by the snapshot.
    pub persona_scope: Vec<String>,
    /// Distilled memory text.
    pub summary: String,
    /// Number of recent events represented.
    pub hot_event_count: usize,
    /// Number of intermediate facts represented.
    pub warm_fact_count: usize,
    /// Number of earlier snapshots represented.
    pub cold_snapshot_count: usize,
    /// Additional structured snapshot metadata.
    pub metadata: HashMap<String, serde_json::Value>,
}
