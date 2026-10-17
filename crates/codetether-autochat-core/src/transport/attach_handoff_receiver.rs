//! Correlated relay attach handoff receiver.
use super::*;

/// Register one bus subscription for a recipient if none exists.
///
/// # Arguments
/// * `receivers` - Recipient-to-handle map.
/// * `bus` - Shared relay bus.
/// * `agent_id` - Recipient identity.
///
/// # Returns
/// Nothing; an existing subscription is preserved.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use codetether_bus_core::AgentBus;
/// use codetether_autochat_core::transport::attach_handoff_receiver;
/// let mut receivers = HashMap::new();
/// attach_handoff_receiver(&mut receivers, AgentBus::new().into_arc(), "worker");
/// assert!(receivers.contains_key("worker"));
/// ```
pub fn attach_handoff_receiver(
    receivers: &mut HashMap<String, BusHandle>,
    bus: Arc<AgentBus>,
    agent_id: &str,
) {
    if receivers.contains_key(agent_id) {
        return;
    }
    receivers.insert(agent_id.to_string(), bus.handle(agent_id.to_string()));
}
