//! access operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// The agent id this handle belongs to.
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    /// Access the agent registry.
    pub fn registry(&self) -> &Arc<registry::AgentRegistry> {
        &self.bus.registry
    }

    /// Access the bus recorder for lossless cursor-based draining.
    pub fn recorder(&self) -> &Arc<recorder::BusRecorder> {
        &self.bus.recorder
    }

    /// Get mutable access to the underlying receiver.
    /// This allows consuming the receiver in stream operations.
    pub fn into_receiver(self) -> broadcast::Receiver<BusEnvelope> {
        self.rx
    }
}
