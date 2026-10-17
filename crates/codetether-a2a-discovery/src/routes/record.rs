//! Metadata for a discovered callable peer.

/// Callable A2A endpoint learned from LAN discovery.
#[derive(Clone)]
pub struct PeerRoute {
    /// Normalized A2A base URL.
    pub endpoint: String,
    /// Peer-scoped bearer capability when advertised.
    pub token: Option<String>,
    /// Human-readable responsibility advertised by the peer.
    pub description: String,
    /// Stable skill identifiers advertised by the peer.
    pub skills: Vec<String>,
    /// Stable provenance identity advertised by the peer.
    pub agent_identity_id: Option<String>,
}
