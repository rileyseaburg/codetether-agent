//! Read-only access to executor collaborators and retry settings.

use super::{ExecutorHost, SwarmExecutor};
use codetether_bus_core::AgentBus;
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_core::result_store::ResultStore;
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::Arc;

impl<H: ExecutorHost> SwarmExecutor<H> {
    /// Returns the runtime control handle when configured.
    pub fn control(&self) -> Option<&SwarmControl> {
        self.inner.control()
    }

    /// Returns the inter-agent bus when configured.
    pub fn bus(&self) -> Option<&Arc<AgentBus>> {
        self.inner.bus()
    }

    /// Returns the coordinator agent when configured.
    pub fn coordinator_agent(&self) -> Option<&Arc<tokio::sync::Mutex<H::Agent>>> {
        self.inner.coordinator_agent()
    }

    /// Clones the shared telemetry collector.
    pub fn telemetry_arc(&self) -> Arc<SwarmTelemetryCollector> {
        self.inner.telemetry_arc()
    }

    /// Returns the shared subtask result store.
    pub fn result_store(&self) -> &Arc<ResultStore> {
        self.inner.result_store()
    }

    /// Returns retry count, initial delay, maximum delay, and multiplier.
    pub fn retry_config(&self) -> (u32, u64, u64, f64) {
        self.inner.retry_config()
    }

    /// Reports whether failed subtask attempts may be retried.
    pub fn retries_enabled(&self) -> bool {
        self.inner.retries_enabled()
    }
}
