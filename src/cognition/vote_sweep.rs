//! Governance sweep resolving all pending proposals for one tick.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::vote_resolve::resolve;
use super::{AttentionItem, PersonaRuntimeState, Proposal, ProposalStatus, SwarmGovernance};

/// Resolve every `Created` proposal, appending any raised attention items.
pub(super) async fn resolve_pending(
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
