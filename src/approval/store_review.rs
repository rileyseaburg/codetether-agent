//! Durable approval decisions that preserve review semantics.

use super::{ApprovalDecision, ApprovalDecisionKind, ApprovalReceipt, ApprovalStore};
use anyhow::{Result, bail};

impl ApprovalStore {
    /// Approve a request and persist whether its authority is reusable.
    pub fn approve_review(
        &self,
        id: &str,
        actor: &str,
        reason: &str,
        kind: ApprovalDecisionKind,
    ) -> Result<ApprovalReceipt> {
        if !kind.approves() {
            bail!("review decision does not approve the request");
        }
        let decision =
            self.record_decision(ApprovalDecision::approve_kind(id, actor, reason, kind))?;
        let request = self
            .request(id)?
            .ok_or_else(|| anyhow::anyhow!("approval request not found"))?;
        Ok(ApprovalReceipt::from_parts(&request, &decision))
    }
}
