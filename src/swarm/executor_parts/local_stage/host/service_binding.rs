//! Native service binding for local stages.
use super::Native;
use crate::swarm::{SwarmConfig, SwarmControl};
use codetether_swarm_core::{SwarmEvent, result_store::ResultStore};
use std::sync::Arc;
use tokio::sync::mpsc;
impl codetether_swarm_local_stage::Services for Native<'_> {
    fn config(&self) -> &SwarmConfig {
        &self.0.config
    }

    fn control(&self) -> Option<&SwarmControl> {
        self.0.control()
    }

    fn events(&self) -> &Option<mpsc::Sender<SwarmEvent>> {
        &self.0.event_tx
    }

    fn bus(&self) -> &Option<Arc<crate::bus::AgentBus>> {
        &self.0.bus
    }

    fn result_store(&self) -> &Arc<ResultStore> {
        &self.0.result_store
    }

    fn try_send_event(&self, event: SwarmEvent) {
        self.0.try_send_event(event);
    }
}
