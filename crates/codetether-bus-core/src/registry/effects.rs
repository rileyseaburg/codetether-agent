//! Side effects applied to the registry when an envelope is published.
//!
//! Keeps [`AgentBus::publish`](crate::AgentBus::publish) small by isolating the
//! registry bookkeeping (ready/shutdown) from the broadcast/record path.

use super::AgentRegistry;
use crate::{BusEnvelope, BusMessage};

/// Apply ready/shutdown registration changes for a published envelope.
///
/// # Arguments
/// * `registry` - Agent-card registry to update.
/// * `envelope` - Published message whose lifecycle event should be applied.
///
/// # Returns
/// Nothing; other message kinds leave the registry unchanged.
pub fn apply(registry: &AgentRegistry, envelope: &BusEnvelope) {
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
