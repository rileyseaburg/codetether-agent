//! Control operations delegate to the one shared ownership table.

use super::ControlHandle;
use crate::session::runtime::CONTROL;

impl ControlHandle {
    /// Cancel this run and wake all attached executors and subscribers.
    pub fn cancel(&self) {
        CONTROL.cancel(&self.0.lease);
    }
    /// Report cancellation or release of this exact run generation.
    pub fn is_cancelled(&self) -> bool {
        CONTROL.is_stopped(&self.0.lease)
    }
    /// Wait for cancellation or release, including an earlier cancellation.
    pub async fn cancelled(&self) {
        if let Some(mut execution) = CONTROL.subscribe(&self.0.lease) {
            execution.cancelled().await;
        }
    }
    /// Hold new orchestration stages without changing durable goal state.
    pub fn pause(&self) {
        CONTROL.pause(&self.0.lease);
    }
    /// Release the transient pause; durable goal holds still apply.
    pub fn resume(&self) {
        CONTROL.resume(&self.0.lease);
    }
    /// Read this run's transient pause state.
    pub fn is_paused(&self) -> bool {
        CONTROL.is_paused(&self.0.lease)
    }
}
