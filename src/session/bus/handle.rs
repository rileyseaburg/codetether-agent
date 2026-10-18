//! Native bus facade preserving the application sink API.
use super::sink::DurableSink;
use crate::session::{Session, SessionEvent};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
/// Session event bus backed by the shared transport implementation.
#[derive(Clone)]
pub struct SessionBus(pub(super) codetether_session_bus::SessionBus<Session>);
impl SessionBus {
    /// Construct a bus with the given broadcast capacity.
    /// # Panics
    /// Panics when capacity is zero.
    pub fn new(capacity: usize) -> Self {
        Self(codetether_session_bus::SessionBus::new(capacity))
    }
    /// Attach the application's durable event sink.
    pub fn with_durable_sink(self, sink: Arc<dyn DurableSink>) -> Self {
        Self(
            self.0
                .with_durable_sink(Arc::new(move |event| sink.write(event))),
        )
    }
    /// Attach a legacy mpsc event receiver.
    pub fn with_legacy_mpsc(self, tx: mpsc::Sender<SessionEvent>) -> Self {
        Self(self.0.with_legacy_mpsc(tx))
    }
    /// Subscribe to all broadcast events.
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> {
        self.0.subscribe()
    }
    /// Return the number of live broadcast subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.0.subscriber_count()
    }
    /// Emit through durability, legacy forwarding and broadcast in order.
    pub fn emit(&self, event: SessionEvent) {
        self.0.emit(event)
    }
}
