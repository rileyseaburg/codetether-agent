//! Shared native executor resources for swarm-run coordination.
use super::{ExecutionHost, SwarmExecutor};
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_core::SwarmConfig;
use codetether_swarm_core::{SwarmEvent, result_store::ResultStore};
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::Arc;
impl<H: ExecutionHost> crate::Services for SwarmExecutor<H> {
    type Planning = H::Planning;
    fn config(&self) -> &SwarmConfig {
        &self.inner.config
    }
    fn control(&self) -> Option<&SwarmControl> {
        self.inner.control.as_ref()
    }
    fn result_store(&self) -> &Arc<ResultStore> {
        &self.inner.result_store
    }
    fn telemetry(&self) -> &Arc<SwarmTelemetryCollector> {
        &self.inner.telemetry
    }
    fn try_send_event(&self, event: SwarmEvent) {
        SwarmExecutor::try_send_event(self, event);
    }
}
