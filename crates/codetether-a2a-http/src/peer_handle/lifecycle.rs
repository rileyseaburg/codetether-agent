//! Abort-on-drop lifecycle for a background peer endpoint.
use super::*;

impl A2APeerHandle {
    /// Abort all background tasks and shut down mDNS immediately.
    pub fn abort(self) {
        drop(self);
    }
}

impl Drop for A2APeerHandle {
    fn drop(&mut self) {
        self.server_task.abort();
        self.discovery_task.abort();
        if let Some(task) = self.mdns_intake_task.as_ref() {
            task.abort();
        }
        self.lan_tasks.iter().for_each(JoinHandle::abort);
        local_identity::deactivate(&self.agent_name);
    }
}
