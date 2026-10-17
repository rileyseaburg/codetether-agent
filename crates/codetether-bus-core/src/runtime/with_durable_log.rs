//! with durable log operations for the shared bus runtime.
use super::*;
impl AgentBus {
    /// Attach a durable log so coordination messages are persisted for replay.
    ///
    /// Builder-style; presence messages ([`super::BusMessage::is_durable`] is
    /// `false`) are never written to the log.
    pub fn with_durable_log(mut self, log: Arc<dyn durable_log::DurableLog>) -> Self {
        self.durable = Some(log);
        self
    }
}
