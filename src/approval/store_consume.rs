//! Terminal consumption of one-time approval decisions.

use super::{ApprovalDecision, ApprovalEvent, ApprovalStatus, ApprovalStore};
use anyhow::{Result, bail};

impl ApprovalStore {
    /// Mark an approved request as consumed so it cannot be replayed.
    ///
    /// # Errors
    ///
    /// Returns an error if the request is missing, not approved, or the
    /// consumption event cannot be appended.
    pub(crate) fn consume(&self, approval_id: &str, actor: &str) -> Result<()> {
        let _lock = self.lock_decisions()?;
        if self.request(approval_id)?.is_none() {
            bail!("approval request not found");
        }
        let decision = self
            .decision(approval_id)?
            .ok_or_else(|| anyhow::anyhow!("approval request is pending"))?;
        match decision.status {
            ApprovalStatus::Approved => {}
            ApprovalStatus::Denied => return Ok(()),
            ApprovalStatus::Pending => bail!("approval request is pending"),
        }
        let consumed = ApprovalDecision::deny(
            approval_id,
            actor,
            "one-time approval consumed after tool execution",
        );
        self.append_event(ApprovalEvent::from(consumed))?;
        Ok(())
    }
}
