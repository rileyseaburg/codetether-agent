//! Local mDNS announcement orchestration.

use super::{DiscoveredPeer, MdnsHandle, browser, scope, service};
use anyhow::{Context, Result};
use mdns_sd::{IfKind, ServiceDaemon};
use std::{net::IpAddr, sync::Arc};
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
/// use codetether_agent::a2a::mdns::announce_and_browse;
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
    let bound_addrs = scope::bounded(bound_addrs)?;
    let daemon = Arc::new(start_daemon(&bound_addrs)?);
    let service =
        service::description(instance_name, bind_port, &bound_addrs, collaboration_token)?;
    let fullname = service.get_fullname().to_string();
    daemon
        .register(service)
        .context("Failed to register mDNS service")?;
    tracing::info!(instance = %instance_name, port = bind_port, "Announced A2A peer over mDNS");
    browser::start(&daemon, fullname.clone(), bind_port, peer_tx)?;
    Ok(MdnsHandle::new(daemon, fullname))
}

fn start_daemon(bound_addrs: &[IpAddr]) -> Result<ServiceDaemon> {
    let daemon = ServiceDaemon::new().context("Failed to start mDNS daemon")?;
    daemon.disable_interface(IfKind::All)?;
    daemon.enable_interface(bound_addrs.to_vec())?;
    Ok(daemon)
}
