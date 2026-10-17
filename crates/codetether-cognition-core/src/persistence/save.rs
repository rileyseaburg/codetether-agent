//! Write a captured state through the existing temporary-file rename path.
use crate::beliefs::Belief;
use crate::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, ThoughtEvent,
};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Capture and persist shared cognition state.
///
/// # Arguments
/// The seven shared stores are snapshotted in parameter order.
/// `resolve_path` is called after capture; `on_saved` runs after the rename.
///
/// # Returns
/// Unit on success, otherwise the existing descriptive error string.
///
/// # Errors
/// Returns an error on directory creation, serialization, write or rename failure.
///
/// # Panics
/// Panics if the resolved path has no parent directory.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc, path::PathBuf};
/// # use tokio::sync::RwLock;
/// # async fn example( personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>, proposals: &Arc<RwLock<HashMap<String, Proposal>>>, beliefs: &Arc<RwLock<HashMap<String, Belief>>>, attention_queue: &Arc<RwLock<Vec<AttentionItem>>>, workspace: &Arc<RwLock<GlobalWorkspace>>, events: &Arc<RwLock<VecDeque<ThoughtEvent>>>, snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>, ) {
/// codetether_cognition_core::persistence::save_state(
///     personas, proposals, beliefs, attention_queue, workspace, events, snapshots,
///     || PathBuf::from(".codetether/cognition/state.json"), |_| {},
/// ).await.unwrap();
/// # }
/// ```
pub async fn save_state(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    proposals: &Arc<RwLock<HashMap<String, Proposal>>>,
    beliefs: &Arc<RwLock<HashMap<String, Belief>>>,
    attention_queue: &Arc<RwLock<Vec<AttentionItem>>>,
    workspace: &Arc<RwLock<GlobalWorkspace>>,
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
    resolve_path: impl FnOnce() -> PathBuf,
    on_saved: impl FnOnce(&Path),
) -> Result<(), String> {
    let state = super::capture::capture(
        personas,
        proposals,
        beliefs,
        attention_queue,
        workspace,
        events,
        snapshots,
    )
    .await;
    let path = resolve_path();
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

    on_saved(&path);
    Ok(())
}
