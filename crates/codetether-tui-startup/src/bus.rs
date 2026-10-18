use std::sync::Arc;

use codetether_bus_core::AgentBus;
use codetether_bus_s3::spawn_bus_s3_sink;

pub(super) fn start() -> Arc<AgentBus> {
    let bus = AgentBus::new().into_arc();
    codetether_bus_core::global::set_global(bus.clone());
    spawn_bus_s3_sink(bus.clone());
    bus
}
