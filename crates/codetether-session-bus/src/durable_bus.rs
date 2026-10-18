//! Typed durable-sink facade over the callback-based event transport.
use crate::{DurableSink, SessionEvent};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
/// Event bus accepting a typed durable sink and preserving transport ordering.
/// # Examples
/// ```
/// let bus = codetether_session_bus::DurableSessionBus::<()>::new(8);
/// assert_eq!(bus.subscriber_count(), 0);
/// ```
#[derive(Clone)]
pub struct DurableSessionBus<S: Clone>(crate::SessionBus<S>);
impl<S: Clone + 'static> DurableSessionBus<S> {
    /// Construct a bus with the given broadcast capacity.
    /// # Panics
    /// Panics when capacity is zero.
    pub fn new(capacity: usize) -> Self {
        Self(crate::SessionBus::new(capacity))
    }
    /// Attach the application's durable event sink.
    pub fn with_durable_sink(self, sink: Arc<dyn DurableSink<S>>) -> Self {
        Self(
            self.0
                .with_durable_sink(Arc::new(move |event| sink.write(event))),
        )
    }
    /// Attach a legacy mpsc event receiver.
    pub fn with_legacy_mpsc(self, tx: mpsc::Sender<SessionEvent<S>>) -> Self {
        Self(self.0.with_legacy_mpsc(tx))
    }
    /// Subscribe to all broadcast events.
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent<S>> {
        self.0.subscribe()
    }
    /// Return the number of live broadcast subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.0.subscriber_count()
    }
    /// Emit through durability, legacy forwarding and broadcast in order.
    pub fn emit(&self, event: SessionEvent<S>) {
        self.0.emit(event)
    }
}
