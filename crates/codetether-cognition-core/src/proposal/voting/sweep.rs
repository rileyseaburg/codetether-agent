//! Governance sweep resolving all pending proposals for one tick.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::resolve;
use crate::{AttentionItem, PersonaRuntimeState, Proposal, ProposalStatus, SwarmGovernance};

/// Resolve every `Created` proposal, appending any raised attention items.
///
/// # Arguments
/// * `proposals` - Shared proposal store.
/// * `personas` - Persona records guarded by the caller or supplied shared lock.
/// * `governance` - Shared voting configuration.
/// * `attention` - Shared attention queue.
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
/// # async fn example(proposals: &Arc<RwLock<HashMap<String, Proposal>>>, personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>, governance: &Arc<RwLock<SwarmGovernance>>, attention: &Arc<RwLock<Vec<AttentionItem>>>, now: DateTime<Utc>,) {
/// let _result = codetether_cognition_core::proposal::voting::resolve_pending(proposals, personas, governance, attention, now).await;
/// # }
/// ```
pub async fn resolve_pending(
    proposals: &Arc<RwLock<HashMap<String, Proposal>>>,
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    governance: &Arc<RwLock<SwarmGovernance>>,
    attention: &Arc<RwLock<Vec<AttentionItem>>>,
    now: DateTime<Utc>,
) {
    let governance = governance.read().await;
    let mut store = proposals.write().await;
    let persona_map = personas.read().await;
    let mut queue = attention.write().await;

    let pending: Vec<String> = store
        .values()
        .filter(|p| p.status == ProposalStatus::Created)
        .map(|p| p.id.clone())
        .collect();

    for id in pending {
        if let Some(proposal) = store.get_mut(&id)
            && let Some(item) = resolve(proposal, &persona_map, &governance, now)
        {
            queue.push(item);
        }
    }
}
