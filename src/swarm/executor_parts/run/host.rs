//! Shared native executor resources for swarm-run coordination.
use super::SwarmExecutor;
use crate::swarm::{SwarmConfig, SwarmControl};
use codetether_swarm_core::{SwarmEvent, result_store::ResultStore};
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::Arc;
mod stages;
pub(super) struct Native<'a>(pub &'a SwarmExecutor);
impl codetether_swarm_runtime::Services for Native<'_> {
    type Planning = crate::swarm::orchestrator::NativeProvider;
    fn config(&self) -> &SwarmConfig {
        &self.0.config
    }
    fn control(&self) -> Option<&SwarmControl> {
        self.0.control.as_ref()
    }
    fn result_store(&self) -> &Arc<ResultStore> {
        &self.0.result_store
    }
    fn telemetry(&self) -> &Arc<SwarmTelemetryCollector> {
        &self.0.telemetry
    }
    fn try_send_event(&self, event: SwarmEvent) {
        self.0.try_send_event(event);
    }
}
