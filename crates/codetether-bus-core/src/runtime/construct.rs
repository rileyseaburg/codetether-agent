//! construct operations for the shared bus runtime.
use super::*;
impl AgentBus {
    /// Create a new bus with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUS_CAPACITY)
    }

    /// Create a new bus with a specific channel capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            registry: Arc::new(registry::AgentRegistry::new()),
            durable: None,
            recorder: Arc::new(recorder::BusRecorder::new(capacity.max(256))),
        }
    }

    /// Wrap `self` in an `Arc` for sharing across tasks.
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
