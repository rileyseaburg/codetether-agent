//! Application entry point for shared Critical-proposal approval.
use super::CognitionRuntime;
use crate::Host;
use anyhow::Result;

impl<H: Host> CognitionRuntime<H> {
    /// Approve a Critical-risk proposal for execution.
    ///
    /// # Errors
    ///
    /// Returns an error when the proposal is unknown, is not Critical risk, or
    /// has not yet reached Verified status.
    pub async fn approve_proposal(&self, proposal_id: &str) -> Result<()> {
        self.state.approve_proposal(proposal_id).await
    }
}
