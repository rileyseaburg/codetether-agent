//! Native application bus binding for the voice API routes.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_voice_api::http::HttpState for AppState<R> {
    fn bus(&self) -> &std::sync::Arc<codetether_bus_core::AgentBus> {
        &self.inner.bus
    }
}
