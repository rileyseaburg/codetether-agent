use std::sync::Arc;

use crate::bus::{AgentBus, s3_sink::spawn_bus_s3_sink};

pub(super) fn start() -> Arc<AgentBus> {
    let bus = AgentBus::new().into_arc();
    crate::bus::set_global(bus.clone());
    spawn_bus_s3_sink(bus.clone());
    bus
}
