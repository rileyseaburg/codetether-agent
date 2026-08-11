//! Data emitted for an mDNS-resolved peer.

/// A peer discovered through mDNS.
///
/// The URLs are ordered by reachability and share the service's advertised
/// port. The token is present for first-party peers that publish one.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::mdns::DiscoveredPeer;
/// let peer = DiscoveredPeer {
///     urls: vec!["http://192.168.1.10:8000".to_string()],
///     instance_name: "agent".to_string(),
///     token: Some("capability".to_string()),
/// };
/// assert_eq!(peer.instance_name, "agent");
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DiscoveredPeer {
    /// Reachability-ordered HTTP endpoints for the service.
    pub urls: Vec<String>,
    /// Service instance name, which matches the peer card name.
    pub instance_name: String,
    /// Optional per-process bearer capability.
    pub token: Option<String>,
}
