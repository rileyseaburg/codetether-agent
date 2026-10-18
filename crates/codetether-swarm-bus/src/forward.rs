//! Bus publication followed by best-effort live-event forwarding.
use codetether_bus_core::AgentBus;
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Publish an executor event to its optional bus and live-event channel.
/// # Arguments
/// * `bus` — Shared agent bus, published before channel delivery.
/// * `event_tx` — Optional bounded live-event destination.
/// * `event` — Event borrowed for bus publication, then moved into the channel.
/// # Returns
/// Returns immediately; a full or closed live-event channel is ignored.
/// # Examples
/// ```
/// codetether_swarm_bus::publish_and_forward(
///     None, None, codetether_swarm_core::SwarmEvent::Error("example".into()),
/// );
/// ```
pub fn publish_and_forward(
    bus: Option<&Arc<AgentBus>>,
    event_tx: Option<&mpsc::Sender<SwarmEvent>>,
    event: SwarmEvent,
) {
    if let Some(bus) = bus {
        crate::publish(bus, &event);
    }
    if let Some(tx) = event_tx {
        let _ = tx.try_send(event);
    }
}
