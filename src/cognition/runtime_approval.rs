//! Application entry point for shared Critical-proposal approval.
use super::CognitionRuntime;
use anyhow::Result;

impl CognitionRuntime {
    /// Approve a Critical-risk proposal for execution.
    ///
    /// # Errors
    ///
    /// Returns an error when the proposal is unknown, is not Critical risk, or
    /// has not yet reached Verified status.
    pub async fn approve_proposal(&self, proposal_id: &str) -> Result<()> {
        codetether_cognition_core::proposal::dispatch::approve_proposal(
            &self.proposals,
            &self.pending_approvals,
            proposal_id,
        )
        .await
    }
}
