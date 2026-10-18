//! Cancellation-safe lifetime for a tool heartbeat task.
use tokio::task::JoinHandle;

/// Cancellation-safe owner for one tool heartbeat task.
/// # Examples
/// See [crate::spawn_heartbeat] for constructing a scoped heartbeat.
pub struct ToolHeartbeat(pub(super) JoinHandle<()>);

impl ToolHeartbeat {
    /// Stop the heartbeat immediately.
    pub fn abort(&self) {
        self.0.abort();
    }
}

impl Drop for ToolHeartbeat {
    fn drop(&mut self) {
        self.0.abort();
    }
}
