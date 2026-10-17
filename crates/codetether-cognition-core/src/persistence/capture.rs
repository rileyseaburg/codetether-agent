//! Snapshot shared stores in the existing lock order.
use super::state::{MAX_RECENT_SNAPSHOTS, SCHEMA_VERSION};
use super::{PersistedCognitionState, evidence::select_evidence, references::referenced_event_ids};
use crate::beliefs::Belief;
use crate::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, ThoughtEvent,
};
use chrono::Utc;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

pub(super) async fn capture(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    proposals: &Arc<RwLock<HashMap<String, Proposal>>>,
    beliefs: &Arc<RwLock<HashMap<String, Belief>>>,
    attention_queue: &Arc<RwLock<Vec<AttentionItem>>>,
    workspace: &Arc<RwLock<GlobalWorkspace>>,
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
) -> PersistedCognitionState {
    let personas_snap = personas.read().await.clone();
    let proposals_snap = proposals.read().await.clone();
    let beliefs_snap = beliefs.read().await.clone();
    let attention_snap = attention_queue.read().await.clone();
    let workspace_snap = workspace.read().await.clone();
    let events_snap = events.read().await.clone();
    let snapshots_snap = snapshots.read().await.clone();

    let ref_ids = referenced_event_ids(&beliefs_snap, &proposals_snap);
    let evidence_events = select_evidence(&events_snap, &ref_ids);
    PersistedCognitionState {
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
    }
}
