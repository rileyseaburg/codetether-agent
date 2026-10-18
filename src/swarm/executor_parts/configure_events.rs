//! Event and telemetry configuration for [`SwarmExecutor`].

use super::{SwarmExecutor, bus_publish};
use crate::telemetry::SwarmTelemetryCollector;
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

impl SwarmExecutor {
    /// Attaches the channel used for real-time swarm events.
    pub fn with_event_tx(mut self, tx: mpsc::Sender<SwarmEvent>) -> Self {
        self.inner.event_tx = Some(tx);
        self
    }

    /// Replaces the default telemetry collector.
    pub fn with_telemetry(mut self, telemetry: Arc<SwarmTelemetryCollector>) -> Self {
        self.inner.telemetry = telemetry;
        self
    }

    pub(super) fn try_send_event(&self, event: SwarmEvent) {
        if let Some(bus) = &self.inner.bus {
            bus_publish::publish(bus, &event);
        }
        if let Some(tx) = &self.inner.event_tx {
            let _ = tx.try_send(event);
        }
    }
}
