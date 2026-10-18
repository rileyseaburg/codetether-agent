//! Native metadata, usage and bus setup for ordinary runs.
use crate::{bus::AgentBus, session::Session};
use std::{path::PathBuf, sync::Arc};
pub(super) struct NativeHost;
impl codetether_run_runtime::Host for NativeHost {
    type Session = Session;
    fn create_bus() -> Arc<AgentBus> {
        let bus = AgentBus::new().into_arc();
        crate::bus::set_global(bus.clone());
        crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
        bus
    }
}
impl codetether_run_runtime::Session for Session {
    fn set_knowledge_snapshot(&mut self, path: Option<PathBuf>) {
        self.metadata.knowledge_snapshot = path;
    }
    fn attach_bus(&mut self, bus: Arc<AgentBus>) {
        self.bus = Some(bus);
    }
    fn usage(&self) -> &crate::provider::Usage {
        &self.usage
    }
}
