//! Maps [`SwarmEvent`]s onto the shared [`AgentBus`] so a running swarm is
//! observable by other agents and tools — not just the local TUI monitor.

use codetether_bus_core::AgentBus;
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;

mod lifecycle;
mod shared;

/// Publish one swarm event to the bus under the `swarm-executor` identity.
///
/// Lifecycle events become task-state updates; high-frequency progress events
/// become tagged `SharedResult`s under the `swarm/` key prefix.
/// # Arguments
/// Receives the shared bus and one borrowed swarm event.
/// # Returns
/// Returns after publishing the mapped event, or ignoring an unsupported detail event.
/// # Examples
/// ```rust,no_run
/// let bus = std::sync::Arc::new(codetether_bus_core::AgentBus::new());
/// codetether_swarm_bus::publish(&bus, &codetether_swarm_core::SwarmEvent::Error("failed".into()));
/// ```
pub fn publish(bus: &Arc<AgentBus>, event: &SwarmEvent) {
    let handle = bus.handle("swarm-executor");
    if !lifecycle::handle(&handle, event) {
        shared::publish_shared(&handle, event);
    }
}

mod payload;
pub use payload::{thinking, tool_output};

/// Per-agent thinking and tool-output messages.
pub mod agent;

mod forward;
pub use forward::publish_and_forward;
