//! Runtime access for cognition workspace.
use codetether_cognition_core::AttentionItem;
use codetether_cognition_core::GlobalWorkspace;
use codetether_cognition_core::SwarmGovernance;
/// Runtime operations needed by workspace HTTP handlers.
/// # Implementors
/// Native runtimes delegate these methods to their existing operations.
pub trait Workspace: Send + Sync {
    /// Access the runtime's existing get attention queue operation.
    fn get_attention_queue(&self) -> impl std::future::Future<Output = Vec<AttentionItem>> + Send;
    /// Access the runtime's existing get workspace operation.
    fn get_workspace(&self) -> impl std::future::Future<Output = GlobalWorkspace> + Send;
    /// Access the runtime's existing get governance operation.
    fn get_governance(&self) -> impl std::future::Future<Output = SwarmGovernance> + Send;
}
