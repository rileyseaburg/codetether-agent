//! Supply the application path and logger to shared persistence.
use super::{log_saved, path::state_path};
use codetether_cognition_core::beliefs::Belief;
use codetether_cognition_core::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, ThoughtEvent,
};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
/// Capture and save cognition stores under the application's data directory.
/// # Arguments
/// The persona, proposal, belief, attention, workspace, event and snapshot
/// stores are passed to the shared capture routine in their existing order.
/// # Returns
/// Unit after the atomic save and success diagnostic complete.
/// # Errors
/// Forwards the shared serializer and filesystem error strings.
/// # Examples
/// See [`codetether_cognition_core::persistence::save_state`] for constructing
/// the seven stores; this entry point supplies their path and diagnostics.
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
