//! construct operations for relay participants.
use super::*;
impl ProtocolRelayRuntime {
    /// Build a relay runtime with an auto-generated relay id.
    pub fn new(bus: Arc<AgentBus>) -> Self {
        Self {
            bus,
            relay_id: format!("relay-{}", &Uuid::new_v4().to_string()[..8]),
        }
    }

    /// Build a relay runtime with an explicit id.
    pub fn with_relay_id(bus: Arc<AgentBus>, relay_id: impl Into<String>) -> Self {
        Self {
            bus,
            relay_id: relay_id.into(),
        }
    }

    /// The relay id used for correlation and topic publication.
    pub fn relay_id(&self) -> &str {
        &self.relay_id
    }
}
