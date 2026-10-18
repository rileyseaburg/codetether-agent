//! Runtime access for cognition lifecycle.
use codetether_cognition_core::MemorySnapshot;
use codetether_cognition_core::ThoughtEvent;
use codetether_cognition_core::control::StartCognitionRequest;
use codetether_cognition_core::status::CognitionStatus;
/// Runtime operations needed by lifecycle HTTP handlers.
/// # Implementors
/// Native runtimes delegate these methods to their existing operations.
pub trait Lifecycle: Send + Sync {
    /// Access the runtime's existing start operation.
    fn start(
        &self,
        req: Option<StartCognitionRequest>,
    ) -> impl std::future::Future<Output = anyhow::Result<CognitionStatus>> + Send;
    /// Access the runtime's existing stop operation.
    fn stop(
        &self,
        reason: Option<String>,
    ) -> impl std::future::Future<Output = anyhow::Result<CognitionStatus>> + Send;
    /// Access the runtime's existing status operation.
    fn status(&self) -> impl std::future::Future<Output = CognitionStatus> + Send;
    /// Access the runtime's existing subscribe events operation.
    fn subscribe_events(&self) -> tokio::sync::broadcast::Receiver<ThoughtEvent>;
    /// Access the runtime's existing latest snapshot operation.
    fn latest_snapshot(&self) -> impl std::future::Future<Output = Option<MemorySnapshot>> + Send;
}
