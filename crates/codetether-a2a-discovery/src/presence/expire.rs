//! Discovery presence expire operation.
use super::*;

/// Periodically expire peers that have stopped re-announcing and emit an
/// `AgentShutdown` + registry deregister for each, so the bus reflects reality.
/// # Arguments
/// * `bus` - Bus and registry from which expired peers are removed.
/// * `liveness` - Shared tracker containing current sightings.
/// # Returns
/// Runs until the task is cancelled.
/// # Examples
/// ```rust,no_run
/// # async fn example(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// # let peers = std::sync::Arc::new(tokio::sync::Mutex::new(codetether_a2a_discovery::liveness::PeerLiveness::new()));
/// codetether_a2a_discovery::presence::expire_loop(bus, peers).await;
/// # }
/// ```
pub async fn expire_loop(bus: Arc<AgentBus>, liveness: Arc<Mutex<PeerLiveness>>) {
    let mut ticker = tokio::time::interval(MDNS_PEER_SWEEP_INTERVAL);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        ticker.tick().await;
        let expired = {
            let mut l = liveness.lock().await;
            l.expire(MDNS_PEER_TTL)
        };
        for card_name in expired {
            crate::routes::remove(&card_name);
            bus.registry.deregister(&card_name);
            bus.handle("a2a-discovery").send(
                "broadcast",
                BusMessage::AgentShutdown {
                    agent_id: card_name.clone(),
                },
            );
            tracing::info!(target: "codetether_agent::a2a::mdns_liveness", peer_name = %card_name, "A2A peer aged out (mDNS TTL expired)");
        }
    }
}
