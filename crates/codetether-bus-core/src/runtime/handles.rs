//! handles operations for the shared bus runtime.
use super::*;
impl AgentBus {
    /// Subscribe to all live envelopes without creating an agent identity.
    ///
    /// # Returns
    /// A receiver positioned at the next publication; prior messages are omitted.
    ///
    /// # Examples
    /// ```
    /// use codetether_bus_core::AgentBus;
    /// let bus = AgentBus::new();
    /// let _receiver = bus.subscribe();
    /// assert_eq!(bus.receiver_count(), 1);
    /// ```
    pub fn subscribe(&self) -> broadcast::Receiver<BusEnvelope> {
        self.tx.subscribe()
    }

    /// Create a `BusHandle` scoped to a specific agent.
    pub fn handle(self: &Arc<Self>, agent_id: impl Into<String>) -> BusHandle {
        BusHandle {
            agent_id: agent_id.into(),
            bus: Arc::clone(self),
            rx: self.tx.subscribe(),
        }
    }

    /// Number of active receivers.
    pub fn receiver_count(&self) -> usize {
        self.tx.receiver_count()
    }
}
