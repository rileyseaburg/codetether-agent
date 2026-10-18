//! Native application bus binding for the voice API routes.
impl codetether_voice_api::http::HttpState for super::AppState {
    fn bus(&self) -> &std::sync::Arc<codetether_bus_core::AgentBus> {
        &self.bus
    }
}
