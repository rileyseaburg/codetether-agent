//! Native cognition proposals bindings.
use crate::cognition::CognitionRuntime;
use codetether_cognition_core::Proposal;
use codetether_cognition_core::execution::DecisionReceipt;
impl codetether_cognition_api::http::Proposals for CognitionRuntime {
    async fn get_proposals(&self) -> std::collections::HashMap<String, Proposal> {
        CognitionRuntime::get_proposals(self).await
    }
    async fn approve_proposal(&self, id: &str) -> anyhow::Result<()> {
        CognitionRuntime::approve_proposal(self, id).await
    }
    async fn get_receipts(&self) -> Vec<DecisionReceipt> {
        CognitionRuntime::get_receipts(self).await
    }
}
