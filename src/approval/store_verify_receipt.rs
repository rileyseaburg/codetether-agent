//! Historical verification for immutable approval receipts.

use super::{ApprovalReceipt, ApprovalStatus, ApprovalStore};
use anyhow::{Result, bail};

impl ApprovalStore {
    /// Verify a historical approval receipt for a tool/action/resource tuple.
    ///
    /// # Errors
    ///
    /// Returns an error if the receipt does not match its immutable approval
    /// decision. Successful verification does not make a consumed grant reusable.
    pub fn verify_receipt(
        &self,
        receipt: &ApprovalReceipt,
        tool: &str,
        action: &str,
        resource: &str,
    ) -> Result<ApprovalReceipt> {
        let request = self
            .request(&receipt.approval_id)?
            .ok_or_else(|| anyhow::anyhow!("approval request not found"))?;
        if !request.matches(tool, action, resource) {
            bail!("approval receipt scope mismatch");
        }
        let decision = self
            .decision_by_id(&receipt.decision_id)?
            .ok_or_else(|| anyhow::anyhow!("approval decision not found"))?;
        if decision.request_id != request.id || !matches!(decision.status, ApprovalStatus::Approved)
        {
            bail!("approval receipt decision mismatch");
        }
        let verified = ApprovalReceipt::from_parts(&request, &decision);
        (&verified == receipt)
            .then_some(verified)
            .ok_or_else(|| anyhow::anyhow!("approval receipt content mismatch"))
    }
}
