//! Native metadata, usage and bus setup for ordinary runs.
use crate::{bus::AgentBus, session::Session};
use std::{path::PathBuf, sync::Arc};
pub(super) type NativeHost = codetether_run_runtime::application::ApplicationHost<Session>;
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
