//! Global workspace refresh during the Compress phase.

use chrono::{DateTime, Utc};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::ranking::rank_attention;
use super::ranking::top_belief_ids;
use super::ranking::top_uncertainties;
use crate::beliefs::Belief;
use crate::{AttentionItem, GlobalWorkspace, ThoughtEvent, ThoughtEventType, ThoughtWorkItem};

/// Recompute the shared workspace from current beliefs and attention.
///
/// # Arguments
/// * `beliefs` - Shared belief store.
/// * `attention` - Shared attention queue.
/// * `workspace` - Shared summary to refresh.
/// * `now` - Time used for this operation.
///
/// # Returns
/// Updates the supplied state; no separate value is returned.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(beliefs: &Arc<RwLock<HashMap<String, Belief>>>, attention: &Arc<RwLock<Vec<AttentionItem>>>, workspace: &Arc<RwLock<GlobalWorkspace>>, now: DateTime<Utc>,) {
/// let _result = codetether_cognition_core::workspace::refresh::refresh_workspace(beliefs, attention, workspace, now).await;
/// # }
/// ```
pub async fn refresh_workspace(
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
///
/// # Arguments
/// * `work` - Work item whose persona and swarm identify the event.
///
/// # Returns
/// The newly constructed event record.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::*;
/// # use codetether_cognition_core::beliefs::Belief;
/// # use chrono::{DateTime, Utc};
/// # use std::{collections::{HashMap, VecDeque}, sync::Arc};
/// # use tokio::sync::{RwLock, broadcast};
/// # async fn example(work: &ThoughtWorkItem) {
/// let _result = codetether_cognition_core::workspace::refresh::updated_event(work);
/// # }
/// ```
pub fn updated_event(work: &ThoughtWorkItem) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::WorkspaceUpdated,
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp: Utc::now(),
        payload: json!({ "updated": true }),
    }
}
