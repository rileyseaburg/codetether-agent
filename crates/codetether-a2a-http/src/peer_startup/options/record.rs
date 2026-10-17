//! Configuration shared by foreground and background peer startup.

/// Inputs for starting an A2A peer runtime.
///
/// Used by `codetether spawn` (one-shot CLI) and by `codetether tui` (which
/// runs the same A2A surface in the background alongside the interactive
/// session). Every field is optional / has an auto-pick default — the
/// "default by default by default" experience is `SpawnOptions::auto()`.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::peer_startup::SpawnOptions;
/// let options = SpawnOptions::auto();
/// assert_eq!(options.port, 0);
/// ```
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
