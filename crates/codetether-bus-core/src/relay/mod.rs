//! Correlated handoffs and participant lifecycle on the shared bus.
//!
//! Construct a [ProtocolRelayRuntime] with a bus, register [RelayAgentProfile]
//! values, and publish handoffs to the recipient and relay observation topics.
use crate::{AgentBus, BusMessage};
use codetether_a2a_core::Part;
use std::sync::Arc;
use uuid::Uuid;

/// Profile for a relay-participating agent.
///
/// # Examples
/// ```
/// use codetether_bus_core::relay::RelayAgentProfile;
/// let agent = RelayAgentProfile { name: "planner".into(), capabilities: vec!["plan".into()] };
/// assert_eq!(agent.name, "planner");
/// ```
#[derive(Debug, Clone)]
pub struct RelayAgentProfile {
    /// Identity used as the sender and registry key.
    pub name: String,
    /// Capabilities announced when this participant registers.
    pub capabilities: Vec<String>,
}

/// Protocol runtime used by auto-chat and other relay-style orchestrators.
///
/// # Examples
/// ```
/// use codetether_bus_core::{AgentBus, relay::ProtocolRelayRuntime};
/// let relay = ProtocolRelayRuntime::with_relay_id(AgentBus::new().into_arc(), "review");
/// assert_eq!(relay.relay_id(), "review");
/// ```
#[derive(Clone)]
pub struct ProtocolRelayRuntime {
    bus: Arc<AgentBus>,
    relay_id: String,
}
mod construct;
mod handoff;
mod registration;
#[cfg(test)]
mod test_register_and_shutdown_agents;
#[cfg(test)]
mod test_send_handoff_emits_agent_and_relay_topics;
