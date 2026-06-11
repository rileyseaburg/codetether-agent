use super::{ApprovalDecision, ApprovalEvent, ApprovalReceipt, ApprovalStatus, ApprovalStore};
use anyhow::{Result, bail};

impl ApprovalStore {
    /// Approve a pending request and return its receipt.
    ///
    /// # Errors
    ///
    /// Returns an error if the request is missing, already decided, or the
    /// decision event cannot be appended.
    pub fn approve(&self, approval_id: &str, actor: &str, reason: &str) -> Result<ApprovalReceipt> {
        let decision =
            self.record_decision(ApprovalDecision::approve(approval_id, actor, reason))?;
        let Some(request) = self.request(approval_id)? else {
            bail!("approval request not found");
        };
        Ok(ApprovalReceipt::from_parts(&request, &decision))
    }

    /// Deny a pending request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request is missing, already decided, or the
    /// decision event cannot be appended.
    pub fn deny(&self, approval_id: &str, actor: &str, reason: &str) -> Result<ApprovalDecision> {
        let decision = ApprovalDecision::deny(approval_id, actor, reason);
        self.record_decision(decision)
    }

    /// Record an approval or denial decision for a pending request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request is missing, already decided, pending was
    /// supplied as a decision, or the event cannot be appended.
    pub fn record_decision(&self, decision: ApprovalDecision) -> Result<ApprovalDecision> {
        if matches!(decision.status, ApprovalStatus::Pending) {
            bail!("pending is not a decision");
        }
        match self.request(&decision.request_id)? {
            Some(_) => {}
            None => bail!("approval request not found"),
        }
        if self.decision(&decision.request_id)?.is_some() {
            bail!("approval request already decided");
        }
        self.append_event(ApprovalEvent::from(decision.clone()))?;
        Ok(decision)
    }
}
