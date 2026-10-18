//! Event and telemetry configuration for [`SwarmExecutor`].

use super::{ExecutorHost, SwarmExecutor};
use codetether_swarm_core::SwarmEvent;
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::Arc;
use tokio::sync::mpsc;

impl<H: ExecutorHost> SwarmExecutor<H> {
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

    /// Publish a lifecycle event to the bus, then try the live event channel.
    /// # Arguments
    /// `event` is the event produced by the application's stage adapter.
    /// # Returns
    /// Completes after publication; full or closed live channels are tolerated.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_swarm_runtime::ExecutorHost>(executor: &codetether_swarm_runtime::SwarmExecutor<H>, event: codetether_swarm_core::SwarmEvent) {
    /// executor.try_send_event(event);
    /// # }
    /// ```
    pub fn try_send_event(&self, event: SwarmEvent) {
        codetether_swarm_bus::publish_and_forward(
            self.inner.bus.as_ref(),
            self.inner.event_tx.as_ref(),
            event,
        );
    }
}
