//! Bus-coupled liveness orchestration for mDNS-discovered A2A peers.
//!
//! [`super::peer_liveness::PeerLiveness`] is the pure tracker; this module
//! wires its decisions to the [`AgentBus`]: it owns the mDNS timing policy,
//! the background expiry sweep (emitting `AgentShutdown` + deregister), and
//! the shared "discovered" heartbeat used by both the mDNS and explicit-seed
//! discovery paths.

use crate::a2a::peer_liveness::PeerLiveness;
use crate::bus::{AgentBus, BusMessage};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

/// Minimum gap between acting on repeat sightings of the same peer. Shorter
/// than mDNS's re-resolution cadence so periodic refreshes pass through, but
/// long enough to collapse the per-interface announcement burst for a single
/// multi-homed peer.
pub const MDNS_PEER_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

/// How long a peer may go silent before it is aged out and an `AgentShutdown`
/// is emitted. Well above the refresh interval so a briefly-missed
/// re-resolution doesn't flap the peer offline.
pub const MDNS_PEER_TTL: Duration = Duration::from_secs(90);

/// How often the expiry sweep runs.
pub const MDNS_PEER_SWEEP_INTERVAL: Duration = Duration::from_secs(30);

/// Broadcast a discovery heartbeat for `name` reachable at `endpoint`.
pub fn emit_discovered(bus: &Arc<AgentBus>, name: &str, endpoint: &str) {
    bus.handle("a2a-discovery").send(
        "broadcast",
        BusMessage::Heartbeat {
            agent_id: name.to_string(),
            status: format!("discovered via A2A at {endpoint}"),
        },
    );
}

/// Periodically expire peers that have stopped re-announcing and emit an
/// `AgentShutdown` + registry deregister for each, so the bus reflects reality.
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
            bus.registry.deregister(&card_name);
            bus.handle("a2a-discovery").send(
                "broadcast",
                BusMessage::AgentShutdown {
                    agent_id: card_name.clone(),
                },
            );
            tracing::info!(peer_name = %card_name, "A2A peer aged out (mDNS TTL expired)");
        }
    }
}

/// Guard that aborts the expiry-loop task when dropped (shutdown, handle drop, etc.).
pub struct AbortOnDrop(tokio::task::JoinHandle<()>);
impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// Spawn the expiry loop and return a guard that aborts it on drop.
pub fn spawn_expire_loop(
    bus: Arc<AgentBus>,
    liveness: Arc<Mutex<PeerLiveness>>,
) -> AbortOnDrop {
    AbortOnDrop(tokio::spawn(expire_loop(bus, liveness)))
}
