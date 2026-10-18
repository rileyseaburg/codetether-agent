//! Ordered durability, legacy forwarding and broadcast delivery.
use crate::{SessionBus, SessionEvent};
impl<S: Clone> SessionBus<S> {
    /// Publish one event.
    ///
    /// For durable events the sink is written **first** (synchronously);
    /// on success the event is then broadcast. A sink failure is logged
    /// at `warn` and does not prevent the broadcast — observers still see
    /// the event even if durable persistence fell over.
    ///
    /// Sending on an empty broadcast channel is not an error; it simply
    /// means no live subscribers.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_session_bus::{SessionBus, SessionEvent};
    /// let bus = SessionBus::<()>::new(4);
    /// bus.emit(SessionEvent::Done); // no subscribers → still ok
    /// # });
    /// ```
    pub fn emit(&self, event: SessionEvent<S>) {
        if event.is_durable()
            && let Err(err) = (self.sink)(&event)
        {
            tracing::warn!(target: "codetether_agent::session::bus::handle", error = %err, "SessionBus: durable sink write failed");
        }

        if let Some(legacy) = self.legacy.as_ref()
            && let Err(err) = legacy.try_send(event.clone())
        {
            tracing::debug!(target: "codetether_agent::session::bus::handle", error = %err, "SessionBus: legacy mpsc forward failed");
        }

        let _ = self.tx.send(event);
    }
}
