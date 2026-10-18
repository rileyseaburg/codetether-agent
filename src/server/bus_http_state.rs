//! Native application bus binding for HTTP event streaming.
impl codetether_bus_sse::http::HttpState for super::AppState {
    fn bus(&self) -> &std::sync::Arc<codetether_bus_core::AgentBus> {
        &self.inner.bus
    }
}
