//! Supply the application path and logger to shared persistence.
use super::{log_saved, path::state_path};
use crate::cognition::beliefs::Belief;
use crate::cognition::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, ThoughtEvent,
};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
pub async fn save_state(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    proposals: &Arc<RwLock<HashMap<String, Proposal>>>,
    beliefs: &Arc<RwLock<HashMap<String, Belief>>>,
    attention_queue: &Arc<RwLock<Vec<AttentionItem>>>,
    workspace: &Arc<RwLock<GlobalWorkspace>>,
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
) -> Result<(), String> {
    codetether_cognition_core::persistence::save_state(
        personas,
        proposals,
        beliefs,
        attention_queue,
        workspace,
        events,
        snapshots,
        state_path,
        log_saved,
    )
    .await
}
