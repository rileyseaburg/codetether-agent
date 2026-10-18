//! Native service binding for local stages.
use super::{LocalStage, LocalStageHost};
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_core::SwarmConfig;
use codetether_swarm_core::{SwarmEvent, result_store::ResultStore};
use std::sync::Arc;
use tokio::sync::mpsc;
impl<H: LocalStageHost> codetether_swarm_local_stage::Services for LocalStage<'_, H> {
    fn config(&self) -> &SwarmConfig {
        &self.0.inner.config
    }

    fn control(&self) -> Option<&SwarmControl> {
        self.0.control()
    }

    fn events(&self) -> &Option<mpsc::Sender<SwarmEvent>> {
        &self.0.inner.event_tx
    }

    fn bus(&self) -> &Option<Arc<codetether_bus_core::AgentBus>> {
        &self.0.inner.bus
    }

    fn result_store(&self) -> &Arc<ResultStore> {
        &self.0.inner.result_store
    }

    fn try_send_event(&self, event: SwarmEvent) {
        self.0.try_send_event(event);
    }
}
