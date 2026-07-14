//! Read-only access to executor collaborators and retry settings.

use super::SwarmExecutor;
use crate::bus::AgentBus;
use crate::swarm::{ResultStore, SwarmControl};
use crate::{agent::Agent, telemetry::SwarmTelemetryCollector};
use std::sync::Arc;

impl SwarmExecutor {
    /// Returns the runtime control handle when configured.
    pub fn control(&self) -> Option<&SwarmControl> {
        self.control.as_ref()
    }

    /// Returns the inter-agent bus when configured.
    pub fn bus(&self) -> Option<&Arc<AgentBus>> {
        self.bus.as_ref()
    }

    /// Returns the coordinator agent when configured.
    pub fn coordinator_agent(&self) -> Option<&Arc<tokio::sync::Mutex<Agent>>> {
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
