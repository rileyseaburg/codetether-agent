//! Side effects applied to the registry when an envelope is published.
//!
//! Keeps [`AgentBus::publish`](super::AgentBus::publish) small by isolating the
//! registry bookkeeping (ready/shutdown) from the broadcast/record path.

use super::{BusEnvelope, BusMessage, registry::AgentRegistry};

/// Apply registry side effects for a published envelope.
pub(super) fn apply(registry: &AgentRegistry, envelope: &BusEnvelope) {
    match &envelope.message {
        BusMessage::AgentReady {
            agent_id,
            capabilities,
        } => {
            registry.register_ready(agent_id, capabilities);
        }
        BusMessage::AgentShutdown { agent_id } => {
            registry.deregister(agent_id);
        }
        _ => {}
    }
}
