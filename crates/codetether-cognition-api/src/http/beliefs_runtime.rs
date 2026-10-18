//! Runtime access for cognition beliefs.
use codetether_cognition_core::beliefs::Belief;
/// Runtime operations needed by beliefs HTTP handlers.
/// # Implementors
/// Native runtimes delegate these methods to their existing operations.
pub trait Beliefs: Send + Sync {
    /// Access the runtime's existing get beliefs operation.
    fn get_beliefs(
        &self,
    ) -> impl std::future::Future<Output = std::collections::HashMap<String, Belief>> + Send;
    /// Access the runtime's existing get belief operation.
    fn get_belief(&self, id: &str) -> impl std::future::Future<Output = Option<Belief>> + Send;
}
