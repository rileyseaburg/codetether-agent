//! Shared human-approval lookup and registration.
use crate::{Proposal, ProposalRisk, ProposalStatus};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Whether a human approved `id`, registering it for approval if not.
pub(super) async fn approved(pending: &Arc<RwLock<HashMap<String, bool>>>, id: &str) -> bool {
    if pending.read().await.get(id).copied().unwrap_or(false) {
        return true;
    }
    pending.write().await.entry(id.to_string()).or_insert(false);
    false
}

/// Approve a known, verified Critical-risk proposal.
///
/// # Errors
/// Rejects unknown proposals, non-Critical risk, and non-Verified status.
///
/// # Arguments
/// * `proposals` - Shared proposal records.
/// * `pending_approvals` - Shared approval decisions indexed by proposal ID.
/// * `proposal_id` - Proposal to approve.
///
/// # Returns
/// Unit when the approval decision has been recorded.
///
/// # Examples
/// ```no_run
/// # use codetether_cognition_core::{*, execution::*, proposal::dispatch::ExecuteCtx};
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use chrono::{DateTime, Utc};
/// # async fn example( proposals: &Arc<RwLock<HashMap<String, Proposal>>>, pending_approvals: &Arc<RwLock<HashMap<String, bool>>>, proposal_id: &str, ) {
/// let _result = codetether_cognition_core::proposal::dispatch::approve_proposal(proposals, pending_approvals, proposal_id).await;
/// # }
/// ```
pub async fn approve_proposal(
    proposals: &Arc<RwLock<HashMap<String, Proposal>>>,
    pending_approvals: &Arc<RwLock<HashMap<String, bool>>>,
    proposal_id: &str,
) -> Result<()> {
    {
        let proposals = proposals.read().await;
        let proposal = proposals
            .get(proposal_id)
            .ok_or_else(|| anyhow!("Proposal not found: {proposal_id}"))?;
        if proposal.risk != ProposalRisk::Critical {
            return Err(anyhow!("Only Critical proposals require human approval"));
        }
        if proposal.status != ProposalStatus::Verified {
            return Err(anyhow!("Proposal is not in Verified status"));
        }
    }
    pending_approvals
        .write()
        .await
        .insert(proposal_id.to_string(), true);
    Ok(())
}
