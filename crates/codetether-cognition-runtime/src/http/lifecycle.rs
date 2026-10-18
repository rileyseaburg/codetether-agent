//! Native cognition lifecycle bindings.
use crate::{CognitionRuntime, Host};
use codetether_cognition_core::MemorySnapshot;
use codetether_cognition_core::ThoughtEvent;
use codetether_cognition_core::control::StartCognitionRequest;
use codetether_cognition_core::status::CognitionStatus;
impl<H: Host> codetether_cognition_api::http::Lifecycle for CognitionRuntime<H>
where
    H::Thinker: Send + Sync,
    H::Registry: Send + Sync,
{
    async fn start(&self, req: Option<StartCognitionRequest>) -> anyhow::Result<CognitionStatus> {
        CognitionRuntime::start(self, req).await
    }
    async fn stop(&self, reason: Option<String>) -> anyhow::Result<CognitionStatus> {
        CognitionRuntime::stop(self, reason).await
    }
    async fn status(&self) -> CognitionStatus {
        CognitionRuntime::status(self).await
    }
    fn subscribe_events(&self) -> tokio::sync::broadcast::Receiver<ThoughtEvent> {
        CognitionRuntime::subscribe_events(self)
    }
    async fn latest_snapshot(&self) -> Option<MemorySnapshot> {
        CognitionRuntime::latest_snapshot(self).await
    }
}
