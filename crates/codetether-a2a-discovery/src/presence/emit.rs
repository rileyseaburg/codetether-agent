//! Discovery presence emit operation.
use super::*;

/// Broadcast a discovery heartbeat for `name` reachable at `endpoint`.
/// # Arguments
/// * `bus` - Bus receiving the discovery heartbeat.
/// * `name` - Discovered agent name.
/// * `endpoint` - Reachable peer endpoint.
/// # Returns
/// Unit after broadcasting the heartbeat.
/// # Examples
/// ```rust,no_run
/// # async fn example(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// # let peers = std::sync::Arc::new(tokio::sync::Mutex::new(codetether_a2a_discovery::liveness::PeerLiveness::new()));
/// codetether_a2a_discovery::presence::emit_discovered(&bus, "peer", "http://peer:4096");
/// # }
/// ```
pub fn emit_discovered(bus: &Arc<AgentBus>, name: &str, endpoint: &str) {
    bus.handle("a2a-discovery").send(
        "broadcast",
        BusMessage::Heartbeat {
            agent_id: name.to_string(),
            status: format!("discovered via A2A at {endpoint}"),
        },
    );
}
