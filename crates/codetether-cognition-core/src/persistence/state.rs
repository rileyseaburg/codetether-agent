//! Schema for stored cognition state.
use crate::beliefs::Belief;
use crate::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, ThoughtEvent,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema-versioned persisted cognition state.
///
/// # Examples
/// ```
/// use codetether_cognition_core::{GlobalWorkspace, persistence::PersistedCognitionState};
/// let state = PersistedCognitionState { schema_version: 1, persisted_at: chrono::Utc::now(),
///     personas: Default::default(), proposals: Default::default(), beliefs: Default::default(),
///     attention_queue: vec![], workspace: GlobalWorkspace::default(),
///     evidence_events: vec![], recent_snapshots: vec![] };
/// assert_eq!(state.schema_version, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedCognitionState {
    /// Version used to reject incompatible stored state.
    pub schema_version: u32,
    /// Time the captured state was assembled.
    pub persisted_at: DateTime<Utc>,
    /// Persona records keyed by ID.
    pub personas: HashMap<String, PersonaRuntimeState>,
    /// Proposal records keyed by ID.
    pub proposals: HashMap<String, Proposal>,
    /// Beliefs keyed by record ID.
    pub beliefs: HashMap<String, Belief>,
    /// Pending and resolved attention records.
    pub attention_queue: Vec<AttentionItem>,
    /// Last shared workspace summary.
    pub workspace: GlobalWorkspace,
    /// Events referenced by beliefs/proposals + last N tail events.
    pub evidence_events: Vec<ThoughtEvent>,
    /// Most recent memory snapshots, newest first.
    pub recent_snapshots: Vec<MemorySnapshot>,
}

pub(super) const SCHEMA_VERSION: u32 = 1;
pub(super) const TAIL_EVENTS: usize = 200;
pub(super) const MAX_RECENT_SNAPSHOTS: usize = 50;
pub(super) const MAX_STATE_FILE_BYTES: u64 = 100 * 1024 * 1024;
