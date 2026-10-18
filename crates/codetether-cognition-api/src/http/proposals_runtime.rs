//! Runtime access for cognition proposals.
use codetether_cognition_core::Proposal;
use codetether_cognition_core::execution::DecisionReceipt;
/// Runtime operations needed by proposals HTTP handlers.
/// # Implementors
/// Native runtimes delegate these methods to their existing operations.
pub trait Proposals: Send + Sync {
    /// Access the runtime's existing get proposals operation.
    fn get_proposals(
        &self,
    ) -> impl std::future::Future<Output = std::collections::HashMap<String, Proposal>> + Send;
    /// Access the runtime's existing approve proposal operation.
    fn approve_proposal(
        &self,
        id: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    /// Access the runtime's existing get receipts operation.
    fn get_receipts(&self) -> impl std::future::Future<Output = Vec<DecisionReceipt>> + Send;
}
