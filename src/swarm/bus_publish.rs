//! Maps [`SwarmEvent`]s onto the shared [`AgentBus`] so a running swarm is
//! observable by other agents and tools — not just the local TUI monitor.

use crate::bus::AgentBus;
use crate::tui::swarm_view::SwarmEvent;
use std::sync::Arc;

#[path = "bus_publish_lifecycle.rs"]
mod lifecycle;
#[path = "bus_publish_shared.rs"]
mod shared;

/// Publish one swarm event to the bus under the `swarm-executor` identity.
///
/// Lifecycle events become task-state updates; high-frequency progress events
/// become tagged `SharedResult`s under the `swarm/` key prefix.
pub(super) fn publish(bus: &Arc<AgentBus>, event: &SwarmEvent) {
    let handle = bus.handle("swarm-executor");
    if !lifecycle::handle(&handle, event) {
        shared::publish_shared(&handle, event);
    }
}
