//! Event and telemetry configuration for [`SwarmExecutor`].

use super::SwarmExecutor;
use crate::telemetry::SwarmTelemetryCollector;
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

impl SwarmExecutor {
    /// Attaches the channel used for real-time swarm events.
    pub fn with_event_tx(mut self, tx: mpsc::Sender<SwarmEvent>) -> Self {
        self.inner = self.inner.with_event_tx(tx);
        self
    }

    /// Replaces the default telemetry collector.
    pub fn with_telemetry(mut self, telemetry: Arc<SwarmTelemetryCollector>) -> Self {
        self.inner = self.inner.with_telemetry(telemetry);
        self
    }

    pub(super) fn try_send_event(&self, event: SwarmEvent) {
        codetether_swarm_bus::publish_and_forward(
            self.inner.bus.as_ref(),
            self.inner.event_tx.as_ref(),
            event,
        );
    }
}
