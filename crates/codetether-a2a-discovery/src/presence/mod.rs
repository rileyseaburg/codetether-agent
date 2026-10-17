//! Peer liveness timing, bus notifications and expiry lifecycle.
//!
//! [emit_discovered] announces sightings; [spawn_expire_loop] owns a background
//! sweep over [PeerLiveness] with [AbortOnDrop] cleanup.
use crate::liveness::PeerLiveness;
use codetether_bus_core::{AgentBus, BusMessage};
use std::{sync::Arc, time::Duration};
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

/// Guard that aborts the expiry-loop task when dropped (shutdown, handle drop, etc.).
///
/// # Examples
/// ```rust,no_run
/// # async fn example(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
/// let peers = std::sync::Arc::new(tokio::sync::Mutex::new(codetether_a2a_discovery::liveness::PeerLiveness::new()));
/// let guard = codetether_a2a_discovery::presence::spawn_expire_loop(bus, peers);
/// drop(guard);
/// # }
/// ```
pub struct AbortOnDrop(tokio::task::JoinHandle<()>);
mod emit;
mod expire;
mod guard;
mod spawn;
pub use emit::emit_discovered;
pub use expire::expire_loop;
pub use spawn::spawn_expire_loop;
