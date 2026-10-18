//! Application-version adapter for shared local A2A discovery.
use anyhow::Result;
pub use codetether_a2a_discovery::mdns::{
    DiscoveredPeer, MdnsHandle, SERVICE_TYPE, sanitize_hostname,
};
use std::net::IpAddr;
use tokio::sync::mpsc;

/// Advertises the local endpoint and starts browsing for A2A peers.
///
/// # Arguments
///
/// * `instance_name` - Unique service instance name.
/// * `bind_port` - Local A2A HTTP port.
/// * `bound_addrs` - Concrete listener interface addresses.
/// * `collaboration_token` - Bearer capability advertised in the TXT record.
/// * `peer_tx` - Channel that receives resolved peers.
///
/// # Returns
///
/// A handle that keeps the mDNS daemon and service registration alive.
///
/// # Errors
///
/// Returns an error when daemon startup, registration, or browsing fails.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_native_a2a::mdns::announce_and_browse;
/// use std::net::{IpAddr, Ipv4Addr};
/// let (tx, _rx) = tokio::sync::mpsc::channel(8);
/// let address = IpAddr::V4(Ipv4Addr::LOCALHOST);
/// let handle = announce_and_browse("agent", 8000, vec![address], "token", tx)?;
/// handle.shutdown();
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn announce_and_browse(
    instance_name: &str,
    bind_port: u16,
    bound_addrs: Vec<IpAddr>,
    collaboration_token: &str,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
) -> Result<MdnsHandle> {
    codetether_a2a_discovery::mdns::announce_and_browse(
        instance_name,
        bind_port,
        bound_addrs,
        collaboration_token,
        peer_tx,
        env!("CARGO_PKG_VERSION"),
    )
}
