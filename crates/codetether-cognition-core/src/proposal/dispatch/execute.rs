//! Verified-proposal sweep with the existing Critical-risk approval gate.
use super::{
    ExecuteCtx,
    approval::approved,
    receipt::{executed_event, receipt_for},
};
use crate::{ProposalRisk, ProposalStatus, ThoughtEvent};

/// Execute every verified proposal, returning the resulting events.
///
/// Critical-risk proposals are registered for human approval and skipped until
/// [`approve_proposal`](super::approve_proposal) is called.
///
/// # Arguments
/// * `ctx` - Shared proposal, receipt and approval stores.
///
/// # Returns
/// Events for verified proposals whose approval requirements are satisfied.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::{*, execution::*, proposal::dispatch::ExecuteCtx};
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example(ctx: ExecuteCtx<'_>) {
/// let _result = codetether_cognition_core::proposal::dispatch::execute_verified(ctx).await;
/// # }
/// ```
pub async fn execute_verified(ctx: ExecuteCtx<'_>) -> Vec<ThoughtEvent> {
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
