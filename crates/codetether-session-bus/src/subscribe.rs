//! Broadcast subscription and active-listener accounting.
use crate::{SessionBus, SessionEvent};
use tokio::sync::broadcast;
impl<S: Clone> SessionBus<S> {
    /// Subscribe to the ephemeral broadcast stream.
    ///
    /// All events — ephemeral and durable — are republished here for live
    /// UIs. Slow consumers will see `RecvError::Lagged(n)`; the durable
    /// sink is unaffected.
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent<S>> {
        self.tx.subscribe()
    }

    /// Current number of active broadcast subscribers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_session_bus::SessionBus;
    ///
    /// let bus = SessionBus::<()>::new(4);
    /// let _rx = bus.subscribe();
    /// assert_eq!(bus.subscriber_count(), 1);
    /// ```
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }
}
