//! Human approval gate for Critical-risk proposals.

use anyhow::{Result, anyhow};

use super::{CognitionRuntime, ProposalRisk, ProposalStatus};

impl CognitionRuntime {
    /// Approve a Critical-risk proposal for execution.
    ///
    /// # Errors
    ///
    /// Returns an error when the proposal is unknown, is not Critical risk, or
    /// has not yet reached Verified status.
    pub async fn approve_proposal(&self, proposal_id: &str) -> Result<()> {
        {
            let proposals = self.proposals.read().await;
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
        self.pending_approvals
            .write()
            .await
            .insert(proposal_id.to_string(), true);
        Ok(())
    }
}
