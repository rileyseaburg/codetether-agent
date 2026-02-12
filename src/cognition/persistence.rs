//! Persistence — atomic save/load of cognition state with evidence preservation.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::beliefs::Belief;
use super::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, ThoughtEvent,
};

/// Schema-versioned persisted cognition state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedCognitionState {
    pub schema_version: u32,
    pub persisted_at: DateTime<Utc>,
    pub personas: HashMap<String, PersonaRuntimeState>,
    pub proposals: HashMap<String, Proposal>,
    pub beliefs: HashMap<String, Belief>,
    pub attention_queue: Vec<AttentionItem>,
    pub workspace: GlobalWorkspace,
    /// Events referenced by beliefs/proposals + last N tail events.
    pub evidence_events: Vec<ThoughtEvent>,
    pub recent_snapshots: Vec<MemorySnapshot>,
}

const SCHEMA_VERSION: u32 = 1;
const TAIL_EVENTS: usize = 200;
const MAX_RECENT_SNAPSHOTS: usize = 50;
const MAX_STATE_FILE_BYTES: u64 = 100 * 1024 * 1024; // 100 MB

/// Get the persistence file path.
fn state_path() -> PathBuf {
    let base = directories::ProjectDirs::from("com", "codetether", "codetether")
        .map(|d| d.data_local_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("/tmp/codetether"));
    base.join("cognition").join("state.json")
}

/// Collect event IDs referenced by beliefs and proposals.
fn referenced_event_ids(
    beliefs: &HashMap<String, Belief>,
    proposals: &HashMap<String, Proposal>,
) -> std::collections::HashSet<String> {
    let mut ids = std::collections::HashSet::new();
    for belief in beliefs.values() {
        for ref_id in &belief.evidence_refs {
            ids.insert(ref_id.clone());
        }
    }
    for proposal in proposals.values() {
        for ref_id in &proposal.evidence_refs {
            ids.insert(ref_id.clone());
        }
    }
    ids
}

/// Trim event payloads for storage (remove very large fields).
fn trim_event_for_storage(event: &ThoughtEvent) -> ThoughtEvent {
    let mut trimmed = event.clone();
    // Trim thinking field if present
    if let Some(thinking) = trimmed.payload.get("thinking").and_then(|v| v.as_str()) {
        if thinking.len() > 500 {
            let short = &thinking[..500];
            trimmed.payload["thinking"] = serde_json::Value::String(format!("{}...", short));
        }
    }
    trimmed
}

/// Save cognition state atomically (tmp + rename).
pub async fn save_state(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    proposals: &Arc<RwLock<HashMap<String, Proposal>>>,
    beliefs: &Arc<RwLock<HashMap<String, Belief>>>,
    attention_queue: &Arc<RwLock<Vec<AttentionItem>>>,
    workspace: &Arc<RwLock<GlobalWorkspace>>,
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
) -> Result<(), String> {
    let personas_snap = personas.read().await.clone();
    let proposals_snap = proposals.read().await.clone();
    let beliefs_snap = beliefs.read().await.clone();
    let attention_snap = attention_queue.read().await.clone();
    let workspace_snap = workspace.read().await.clone();
    let events_snap = events.read().await.clone();
    let snapshots_snap = snapshots.read().await.clone();

    // Collect referenced events + tail
    let ref_ids = referenced_event_ids(&beliefs_snap, &proposals_snap);
    let mut evidence_events: Vec<ThoughtEvent> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Referenced events first
    for event in events_snap.iter() {
        if ref_ids.contains(&event.id) && seen.insert(event.id.clone()) {
            evidence_events.push(trim_event_for_storage(event));
        }
    }

    // Then tail events
    let tail_start = events_snap.len().saturating_sub(TAIL_EVENTS);
    for event in events_snap.iter().skip(tail_start) {
        if seen.insert(event.id.clone()) {
            evidence_events.push(trim_event_for_storage(event));
        }
    }

    let state = PersistedCognitionState {
        schema_version: SCHEMA_VERSION,
        persisted_at: Utc::now(),
        personas: personas_snap,
        proposals: proposals_snap,
        beliefs: beliefs_snap,
        attention_queue: attention_snap,
        workspace: workspace_snap,
        evidence_events,
        recent_snapshots: snapshots_snap
            .into_iter()
            .rev()
            .take(MAX_RECENT_SNAPSHOTS)
            .collect(),
    };

    let path = state_path();
    let dir = path.parent().unwrap();

    // Create directory if needed
    if let Err(e) = tokio::fs::create_dir_all(dir).await {
        return Err(format!("Failed to create persistence directory: {}", e));
    }

    // Serialize
    let json = match serde_json::to_string_pretty(&state) {
        Ok(j) => j,
        Err(e) => return Err(format!("Failed to serialize state: {}", e)),
    };

    // Atomic write: tmp + rename
    let tmp_path = path.with_extension("json.tmp");
    if let Err(e) = tokio::fs::write(&tmp_path, &json).await {
        return Err(format!("Failed to write temp file: {}", e));
    }
    if let Err(e) = tokio::fs::rename(&tmp_path, &path).await {
        return Err(format!("Failed to rename temp file: {}", e));
    }

    tracing::debug!(path = %path.display(), "Cognition state persisted");
    Ok(())
}

/// Load persisted cognition state, if available.
pub fn load_state() -> Option<PersistedCognitionState> {
    let path = state_path();
    if !path.exists() {
        return None;
    }

    // Guard against bloated state files that would OOM the process
    match std::fs::metadata(&path) {
        Ok(meta) if meta.len() > MAX_STATE_FILE_BYTES => {
            tracing::warn!(
                size_mb = meta.len() / (1024 * 1024),
                max_mb = MAX_STATE_FILE_BYTES / (1024 * 1024),
                "Persisted cognition state too large, starting fresh"
            );
            let _ = std::fs::rename(&path, path.with_extension("json.bloated"));
            return None;
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to stat persisted cognition state");
            return None;
        }
        _ => {}
    }

    let data = match std::fs::read_to_string(&path) {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!(error = %e, "Failed to read persisted cognition state");
            return None;
        }
    };

    match serde_json::from_str::<PersistedCognitionState>(&data) {
        Ok(state) => {
            if state.schema_version != SCHEMA_VERSION {
                tracing::warn!(
                    persisted_version = state.schema_version,
                    current_version = SCHEMA_VERSION,
                    "Schema version mismatch, starting fresh"
                );
                return None;
            }
            tracing::info!(
                persisted_at = %state.persisted_at,
                personas = state.personas.len(),
                beliefs = state.beliefs.len(),
                "Loaded persisted cognition state"
            );
            Some(state)
        }
        Err(e) => {
            tracing::warn!(error = %e, "Corrupt persisted cognition state, starting fresh");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_serialize() {
        let state = PersistedCognitionState {
            schema_version: SCHEMA_VERSION,
            persisted_at: Utc::now(),
            personas: HashMap::new(),
            proposals: HashMap::new(),
            beliefs: HashMap::new(),
            attention_queue: Vec::new(),
            workspace: GlobalWorkspace::default(),
            evidence_events: Vec::new(),
            recent_snapshots: Vec::new(),
        };

        let json = serde_json::to_string(&state).expect("should serialize");
        let loaded: PersistedCognitionState =
            serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(loaded.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn referenced_events_collected() {
        use super::super::beliefs::{Belief, BeliefStatus};
        use chrono::Duration;

        let mut beliefs = HashMap::new();
        beliefs.insert(
            "b1".to_string(),
            Belief {
                id: "b1".to_string(),
                belief_key: "test".to_string(),
                claim: "test".to_string(),
                confidence: 0.8,
                evidence_refs: vec!["evt-1".to_string(), "evt-2".to_string()],
                asserted_by: "p1".to_string(),
                confirmed_by: Vec::new(),
                contested_by: Vec::new(),
                contradicts: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                review_after: Utc::now() + Duration::hours(1),
                status: BeliefStatus::Active,
            },
        );

        let proposals = HashMap::new();
        let ids = referenced_event_ids(&beliefs, &proposals);
        assert!(ids.contains("evt-1"));
        assert!(ids.contains("evt-2"));
        assert_eq!(ids.len(), 2);
    }
}
