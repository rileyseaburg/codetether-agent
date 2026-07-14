//! Event and telemetry configuration for [`SwarmExecutor`].

use super::{SwarmExecutor, bus_publish};
use crate::telemetry::SwarmTelemetryCollector;
use crate::tui::swarm_view::SwarmEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

impl SwarmExecutor {
    /// Attaches the channel used for real-time swarm events.
    pub fn with_event_tx(mut self, tx: mpsc::Sender<SwarmEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    /// Replaces the default telemetry collector.
    pub fn with_telemetry(mut self, telemetry: Arc<SwarmTelemetryCollector>) -> Self {
        self.telemetry = telemetry;
        self
    }

    pub(super) fn try_send_event(&self, event: SwarmEvent) {
        if let Some(bus) = &self.bus {
            bus_publish::publish(bus, &event);
        }
        if let Some(tx) = &self.event_tx {
            let _ = tx.try_send(event);
        }
    }
}
