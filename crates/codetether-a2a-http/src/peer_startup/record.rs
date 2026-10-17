//! Prepared listener, router and discovery ownership.
use super::*;
/// A bound endpoint and its discovery tasks, ready for foreground or background serving.
///
/// # Examples
///
/// ```rust,no_run
/// # fn inspect(prepared: codetether_a2a_http::peer_startup::A2APreparation) {
/// assert!(!prepared.public_url.is_empty());
/// # }
/// ```

pub struct A2APreparation {
    /// Bound listener with the effective port.
    pub listener: tokio::net::TcpListener,
    /// Application router attached to the peer.
    pub router: Router,
    /// Explicit-seed discovery task.
    pub discovery_task: tokio::task::JoinHandle<()>,
    /// Optional local-discovery intake task.
    pub mdns_intake_task: Option<tokio::task::JoinHandle<()>>,
    /// LAN announcement and listening tasks.
    pub lan_tasks: Vec<tokio::task::JoinHandle<()>>,
    /// Owned mDNS advertisement and browser.
    pub mdns_handle: Option<mdns::MdnsHandle>,
    /// Advertised local agent name.
    pub agent_name: String,
    /// Effective listener address.
    pub bind_addr: String,
    /// Normalized advertised URL.
    pub public_url: String,
}
