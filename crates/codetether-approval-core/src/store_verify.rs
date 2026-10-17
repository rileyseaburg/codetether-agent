use super::{ApprovalReceipt, ApprovalStatus, ApprovalStore};
use anyhow::{Result, bail};

impl ApprovalStore {
    /// Verify an approved id for a tool/action/resource tuple.
    ///
    /// # Errors
    ///
    /// Returns an error if the id is unknown, pending, denied, mismatched, or
    /// the JSONL log cannot be read.
    pub fn verify(
        &self,
        approval_id: &str,
        tool: &str,
        action: &str,
        resource: &str,
    ) -> Result<ApprovalReceipt> {
        let request = self
            .request(approval_id)?
            .ok_or_else(|| anyhow::anyhow!("approval request not found"))?;
        if !request.matches(tool, action, resource) {
            bail!("approval does not match requested tool/action/resource");
        }
        let decision = self
            .decision(approval_id)?
            .ok_or_else(|| anyhow::anyhow!("approval request is pending"))?;
        match decision.status {
            ApprovalStatus::Approved => Ok(ApprovalReceipt::from_parts(&request, &decision)),
            ApprovalStatus::Denied => bail!("approval request was denied"),
            ApprovalStatus::Pending => bail!("approval request is pending"),
        }
    }

    /// Verify an approval receipt for a tool/action/resource tuple.
    ///
    /// # Errors
    ///
    /// Returns an error if the receipt no longer matches the approved decision
    /// or if [`Self::verify`] fails.
    pub fn verify_receipt(
        &self,
        receipt: &ApprovalReceipt,
        tool: &str,
        action: &str,
        resource: &str,
    ) -> Result<ApprovalReceipt> {
        let verified = self.verify(&receipt.approval_id, tool, action, resource)?;
        if verified.decision_id != receipt.decision_id {
            bail!("approval receipt decision mismatch");
        }
        Ok(verified)
    }
}
