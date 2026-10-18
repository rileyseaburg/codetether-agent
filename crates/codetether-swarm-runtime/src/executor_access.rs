//! Read-only access to shared executor collaborators and retry settings.

use crate::ExecutorState;
use codetether_bus_core::AgentBus;
use codetether_session_control::ControlHandle;
use codetether_swarm_core::result_store::ResultStore;
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::Arc;

impl<A, C> ExecutorState<A, C> {
    /// Returns the runtime control handle when configured.
    pub fn control(&self) -> Option<&ControlHandle> {
        self.control.as_ref()
    }

    /// Returns the inter-agent bus when configured.
    pub fn bus(&self) -> Option<&Arc<AgentBus>> {
        self.bus.as_ref()
    }

    /// Returns the coordinator agent when configured.
    pub fn coordinator_agent(&self) -> Option<&Arc<tokio::sync::Mutex<A>>> {
        self.coordinator_agent.as_ref()
    }

    /// Clones the shared telemetry collector.
    pub fn telemetry_arc(&self) -> Arc<SwarmTelemetryCollector> {
        Arc::clone(&self.telemetry)
    }

    /// Returns the shared subtask result store.
    pub fn result_store(&self) -> &Arc<ResultStore> {
        &self.result_store
    }

    /// Returns retry count, initial delay, maximum delay, and multiplier.
    pub fn retry_config(&self) -> (u32, u64, u64, f64) {
        (
            self.config.max_retries,
            self.config.base_delay_ms,
            self.config.max_delay_ms,
            2.0,
        )
    }

    /// Reports whether failed subtask attempts may be retried.
    pub fn retries_enabled(&self) -> bool {
        self.config.max_retries > 0
    }
}
