use super::Host;
use codetether_bus_core::BusMessage;

/// Update app state based on a bus message (agent presence tracking).
///
/// Heartbeats are how A2A/mDNS-discovered peers announce themselves, so they
/// register too. `AgentShutdown` removes a peer (used by mDNS TTL expiry).
/// Apply recorded bus input through frontend bindings.
/// # Arguments
/// * `host` — Frontend identities, observers and presentation effects.
/// * Input — Recorded envelope, message or recorder to process.
/// # Returns
/// Updates only the state selected by the bus message and routing rules.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_bus_view::ingress::Host, message: &codetether_bus_core::BusMessage) {
/// codetether_bus_view::ingress::track(host, message);
/// # }
/// ```
pub fn track(host: &mut impl Host, message: &BusMessage) {
    host.observe_tasks(message);
    match message {
        BusMessage::AgentReady { agent_id, .. } | BusMessage::Heartbeat { agent_id, .. } => {
            host.agents().insert(agent_id.clone());
        }
        BusMessage::AgentShutdown { agent_id } => {
            host.agents().remove(agent_id.as_str());
        }
        _ => {}
    }
}
