//! Execution of verified proposals, with a human gate for Critical risk.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::executor::DecisionReceipt;
use super::proposal_receipt::{executed_event, receipt_for};
use super::{Proposal, ProposalRisk, ProposalStatus, ThoughtEvent};

/// Shared handles the execution sweep needs.
pub(super) struct ExecuteCtx<'a> {
    pub proposals: &'a Arc<RwLock<HashMap<String, Proposal>>>,
    pub receipts: &'a Arc<RwLock<Vec<DecisionReceipt>>>,
    pub pending_approvals: &'a Arc<RwLock<HashMap<String, bool>>>,
}

/// Execute every verified proposal, returning the resulting events.
///
/// Critical-risk proposals are registered for human approval and skipped until
/// [`approve_proposal`](super::CognitionRuntime::approve_proposal) is called.
pub(super) async fn execute_verified(ctx: ExecuteCtx<'_>) -> Vec<ThoughtEvent> {
    let mut events = Vec::new();
    let mut store = ctx.proposals.write().await;
    let verified: Vec<String> = store
        .values()
        .filter(|p| p.status == ProposalStatus::Verified)
        .map(|p| p.id.clone())
        .collect();

    for id in verified {
        let Some(proposal) = store.get_mut(&id) else {
            continue;
        };
        if proposal.risk == ProposalRisk::Critical && !approved(ctx.pending_approvals, &id).await {
            continue;
        }
        let receipt = receipt_for(proposal, &id);
        events.push(executed_event(proposal, &receipt, &id));
        ctx.receipts.write().await.push(receipt);
        proposal.status = ProposalStatus::Executed;
    }
    events
}

/// Whether a human approved `id`, registering it for approval if not.
async fn approved(pending: &Arc<RwLock<HashMap<String, bool>>>, id: &str) -> bool {
    if pending.read().await.get(id).copied().unwrap_or(false) {
        return true;
    }
    pending.write().await.entry(id.to_string()).or_insert(false);
    false
}
