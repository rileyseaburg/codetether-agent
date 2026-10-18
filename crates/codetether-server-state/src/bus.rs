//! Native application bus binding for HTTP event streaming.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_bus_sse::http::HttpState for AppState<R> {
    fn bus(&self) -> &std::sync::Arc<codetether_bus_core::AgentBus> {
        &self.inner.bus
    }
}
