//! Discovery presence spawn operation.
use super::*;

/// Spawn the expiry loop and return a guard that aborts it on drop.
/// # Arguments
/// * `bus` - Bus and registry for expiry updates.
/// * `liveness` - Shared peer tracker.
/// # Returns
/// A guard that aborts the spawned sweep when dropped.
/// # Panics
/// Panics if called outside a Tokio runtime.
/// # Examples
/// ```rust,no_run
/// # async fn example(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// # let peers = std::sync::Arc::new(tokio::sync::Mutex::new(codetether_a2a_discovery::liveness::PeerLiveness::new()));
/// let guard = codetether_a2a_discovery::presence::spawn_expire_loop(bus, peers);
/// drop(guard);
/// # }
/// ```
pub fn spawn_expire_loop(bus: Arc<AgentBus>, liveness: Arc<Mutex<PeerLiveness>>) -> AbortOnDrop {
    AbortOnDrop(tokio::spawn(expire_loop(bus, liveness)))
}
