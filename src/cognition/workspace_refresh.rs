//! Global workspace refresh during the Compress phase.

use chrono::{DateTime, Utc};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::beliefs::Belief;
use super::workspace_attention::rank_attention;
use super::workspace_rank::top_belief_ids;
use super::workspace_uncertain::top_uncertainties;
use super::{AttentionItem, GlobalWorkspace, ThoughtEvent, ThoughtEventType, ThoughtWorkItem};

/// Recompute the shared workspace from current beliefs and attention.
pub(super) async fn refresh_workspace(
    beliefs: &Arc<RwLock<HashMap<String, Belief>>>,
    attention: &Arc<RwLock<Vec<AttentionItem>>>,
    workspace: &Arc<RwLock<GlobalWorkspace>>,
    now: DateTime<Utc>,
) {
    let store = beliefs.read().await;
    let queue = attention.read().await;
    let (top_beliefs, uncertainties, top_attention) = (
        top_belief_ids(&store, now),
        top_uncertainties(&store),
        rank_attention(&queue),
    );

    let mut ws = workspace.write().await;
    ws.top_beliefs = top_beliefs;
    ws.top_uncertainties = uncertainties;
    ws.top_attention = top_attention;
    ws.updated_at = now;
}

/// Event announcing that the workspace was refreshed.
pub(super) fn updated_event(work: &ThoughtWorkItem) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::WorkspaceUpdated,
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp: Utc::now(),
        payload: json!({ "updated": true }),
    }
}
