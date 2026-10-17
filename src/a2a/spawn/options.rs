//! Peer startup options.
use super::*;

/// Inputs for starting an A2A peer runtime.
///
/// Used by `codetether spawn` (one-shot CLI) and by `codetether tui` (which
/// runs the same A2A surface in the background alongside the interactive
/// session). Every field is optional / has an auto-pick default — the
/// "default by default by default" experience is `SpawnOptions::auto()`.
#[derive(Debug, Clone)]
pub struct SpawnOptions {
    /// Agent name. `None` → auto-pick `<host>-<repo>-<short-pid>`.
    pub name: Option<String>,
    /// Bind hostname. `0.0.0.0` is the zero-config default so mDNS can
    /// announce real interfaces. Use `127.0.0.1` for loopback-only peers.
    pub hostname: String,
    /// Bind port. `0` → OS picks an available port.
    pub port: u16,
    /// Public URL published in the agent card. `None` → derived from the
    /// effective bound address after binding.
    pub public_url: Option<String>,
    /// Optional card description.
    pub description: Option<String>,
    /// Explicit peer seeds — used in addition to mDNS discovery, mainly
    /// for cross-host setups where mDNS isn't routable.
    pub peer: Vec<String>,
    /// Polling interval for explicit peer seeds (mDNS is event-driven).
    /// Clamped to ≥ 5.
    pub discovery_interval_secs: u64,
    /// Send a non-blocking intro `message/send` on first sighting of a peer.
    pub auto_introduce: bool,
    /// Enable mDNS announce + browse (true P2P discovery, no central state).
    pub mdns: bool,
}

impl SpawnOptions {
    /// Zero-config defaults: wildcard host, auto port/name, mDNS, intro on.
    pub fn auto() -> Self {
        Self {
            name: None,
            hostname: "0.0.0.0".to_string(),
            port: 0,
            public_url: None,
            description: None,
            peer: Vec::new(),
            discovery_interval_secs: 15,
            auto_introduce: true,
            mdns: true,
        }
    }

    /// Materialize options from the `codetether spawn` CLI args.
    pub fn from_spawn_args(args: &SpawnArgs) -> Self {
        Self {
            name: args.name.clone(),
            hostname: args.hostname.clone(),
            port: args.port,
            public_url: args.public_url.clone(),
            description: args.description.clone(),
            peer: args.peer.clone(),
            discovery_interval_secs: args.discovery_interval_secs,
            auto_introduce: args.auto_introduce,
            mdns: args.mdns,
        }
    }
}
