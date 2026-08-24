//! Atomic claiming of exact one-time approval grants.

use super::{ApprovalDecision, ApprovalEvent, ApprovalReceipt, ApprovalStatus, ApprovalStore};
use anyhow::{Result, bail};

impl ApprovalStore {
    pub(crate) fn claim(
        &self,
        approval_id: &str,
        tool: &str,
        action: &str,
        resource: &str,
        actor: &str,
    ) -> Result<ApprovalReceipt> {
        let _lock = self.lock_decisions()?;
        let request = self
            .request(approval_id)?
            .ok_or_else(|| anyhow::anyhow!("approval request not found"))?;
        if request.tool != tool {
            bail!("approval tool mismatch");
        }
        if request.action != action {
            bail!("approval action mismatch");
        }
        if request.resource != resource {
            bail!("approval resource mismatch");
        }
        let decision = self
            .decision(approval_id)?
            .ok_or_else(|| anyhow::anyhow!("approval request is pending"))?;
        match decision.status {
            ApprovalStatus::Approved => {}
            ApprovalStatus::Denied => bail!("approval request denied"),
            ApprovalStatus::Pending => bail!("approval request is pending"),
        }
        let receipt = ApprovalReceipt::from_parts(&request, &decision);
        let consumed = ApprovalDecision::deny(
            approval_id,
            actor,
            "one-time approval atomically claimed for tool execution",
        );
        self.append_event(ApprovalEvent::from(consumed))?;
        Ok(receipt)
    }
}
