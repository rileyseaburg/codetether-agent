use super::super::CognitionRuntime;
use crate::{MemorySnapshot, ThoughtEvent};
use std::sync::Arc;
use tokio::sync::broadcast;
impl<T, R> CognitionRuntime<T, R> {
    /// Whether cognition is enabled by feature flag.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Subscribe to thought events for streaming.
    pub fn subscribe_events(&self) -> broadcast::Receiver<ThoughtEvent> {
        self.event_tx.subscribe()
    }

    /// Set the tool registry for capability-based tool execution.
    pub fn set_tools(&mut self, registry: Arc<R>) {
        self.tools = Some(registry);
    }

    /// Return the most recent memory snapshot, if any.
    pub async fn latest_snapshot(&self) -> Option<MemorySnapshot> {
        self.snapshots.read().await.back().cloned()
    }
}
