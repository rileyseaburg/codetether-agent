//! mDNS / DNS-SD discovery for true peer-to-peer A2A.
//!
//! Each A2A peer announces itself as a `_codetether-a2a._tcp.local.` service
//! and browses for the same service type. Peers find each other on the LAN
//! (and on loopback for single-machine multi-agent setups) without any
//! central registry, broker, or seed file.
//!
//! # Service shape
//!
//! - Service type: `_codetether-a2a._tcp.local.`
//! - Instance name: the agent's card name (must be unique on the LAN; the
//!   default name template `<host>-<repo>-<short-pid>` ensures this).
//! - Port: the agent's bound A2A port.
//! - TXT records:
//!   - `name=<card-name>`
//!   - `path=/` (JSON-RPC root)
//!   - `protocol=a2a-jsonrpc`
//!   - `version=<crate version>`
//!
//! # Why mDNS
//!
//! - **No central state.** No file, no broker, no seed list — true P2P.
//! - **Zero-config.** Two `codetether tui` processes on one box, or two
//!   on the same LAN, find each other on launch.
//! - **Standard.** Same protocol as Bonjour/Avahi/Chromecast/AirPlay.
//!
//! # Loopback
//!
//! `mdns-sd` enables the loopback interface by default in addition to all
//! detected non-loopback interfaces, so two agents bound to `127.0.0.1`
//! on the same host will discover each other.

use anyhow::{Context, Result};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::mpsc;

/// mDNS service type used by all CodeTether A2A peers.
pub const SERVICE_TYPE: &str = "_codetether-a2a._tcp.local.";

/// Handle to a registered mDNS service. Drop or call [`Self::shutdown`] to
/// unregister and stop the daemon.
pub struct MdnsHandle {
    daemon: Arc<ServiceDaemon>,
    fullname: String,
}

impl MdnsHandle {
    /// Best-effort unregister and shut the daemon down.
    pub fn shutdown(self) {
        // Calling unregister is best-effort; the daemon may already be
        // gone if shutdown_signal arrived twice.
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}

impl Drop for MdnsHandle {
    fn drop(&mut self) {
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}

/// A peer discovered over mDNS — just a URL, the existing A2A discovery
/// flow takes it from there (fetches the agent card, registers, intros).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoveredPeer {
    /// `http://<addr>:<port>` — base URL the agent card lives under.
    pub url: String,
    /// Service instance name reported by mDNS (== card name).
    pub instance_name: String,
}

/// Announce ourselves on mDNS and start a browse loop that emits every
/// resolved peer (other than ourselves) onto `peer_tx`.
///
/// Returns once the service is registered and the browse loop is running
/// in a background tokio task. The returned [`MdnsHandle`] keeps the
/// daemon alive — drop it on shutdown.
pub fn announce_and_browse(
    instance_name: &str,
    bind_port: u16,
    bound_addrs: Vec<IpAddr>,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
) -> Result<MdnsHandle> {
    let daemon = ServiceDaemon::new().context("Failed to start mDNS daemon")?;
    let daemon = Arc::new(daemon);

    // Properties surfaced in TXT records so peers can sanity-check us.
    let mut props: HashMap<String, String> = HashMap::new();
    props.insert("name".to_string(), instance_name.to_string());
    props.insert("path".to_string(), "/".to_string());
    props.insert("protocol".to_string(), "a2a-jsonrpc".to_string());
    props.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());

    // mdns-sd's hostname must end with `.local.` and be unique per service
    // instance on the LAN. We derive it from the instance name.
    let mdns_hostname = format!("{}.local.", sanitize_hostname(instance_name));

    let addrs: Vec<IpAddr> = if bound_addrs.is_empty() {
        // No specific bind addrs given — let mdns-sd pick from interfaces.
        // We must still pass at least one address; use loopback so the
        // service registers, and rely on mdns-sd's interface enumeration
        // for the actual broadcast.
        vec!["127.0.0.1".parse().unwrap()]
    } else {
        bound_addrs.clone()
    };

    let service = ServiceInfo::new(
        SERVICE_TYPE,
        instance_name,
        &mdns_hostname,
        addrs.as_slice(),
        bind_port,
        Some(props),
    )
    .context("Failed to construct mDNS ServiceInfo")?
    .enable_addr_auto();

    let fullname = service.get_fullname().to_string();
    daemon
        .register(service)
        .context("Failed to register mDNS service")?;

    tracing::info!(
        instance = %instance_name,
        port = bind_port,
        service_type = SERVICE_TYPE,
        "Announced A2A peer over mDNS"
    );

    // Browse for the same service type. The daemon returns a flume receiver
    // (sync) — we spawn a blocking task that forwards events into our async
    // channel.
    let receiver = daemon
        .browse(SERVICE_TYPE)
        .context("Failed to start mDNS browse")?;
    let self_fullname = fullname.clone();
    let self_port = bind_port;
    tokio::task::spawn_blocking(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::SearchStarted(svc) => {
                    tracing::debug!(service = %svc, "mDNS browse search started");
                }
                ServiceEvent::ServiceFound(svc, fullname) => {
                    tracing::debug!(service = %svc, fullname = %fullname, "mDNS service found");
                }
                ServiceEvent::ServiceResolved(info) => {
                    let info_fullname = info.get_fullname().to_string();
                    tracing::debug!(
                        fullname = %info_fullname,
                        port = info.get_port(),
                        addrs = ?info.get_addresses(),
                        "mDNS service resolved"
                    );
                    if info_fullname == self_fullname {
                        continue;
                    }
                    let port = info.get_port();
                    let Some(addr) = info.get_addresses().iter().find(|a| a.is_ipv4()).copied()
                    else {
                        continue;
                    };
                    if port == self_port && addr.is_loopback() {
                        continue;
                    }
                    let url = format!("http://{addr}:{port}");
                    let instance = info_fullname
                        .strip_suffix(SERVICE_TYPE)
                        .unwrap_or(&info_fullname)
                        .trim_end_matches('.')
                        .to_string();
                    let peer = DiscoveredPeer {
                        url,
                        instance_name: instance,
                    };
                    if peer_tx.blocking_send(peer).is_err() {
                        break;
                    }
                }
                ServiceEvent::ServiceRemoved(svc, fullname) => {
                    tracing::debug!(service = %svc, fullname = %fullname, "mDNS service removed");
                }
                ServiceEvent::SearchStopped(svc) => {
                    tracing::debug!(service = %svc, "mDNS browse search stopped");
                }
            }
        }
    });

    Ok(MdnsHandle { daemon, fullname })
}

/// Replace characters that aren't valid in DNS hostnames with `-`.
fn sanitize_hostname(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sanitize_hostname;

    #[test]
    fn sanitize_hostname_preserves_alnum_and_dash() {
        assert_eq!(sanitize_hostname("alice-7"), "alice-7");
    }

    #[test]
    fn sanitize_hostname_replaces_dots_and_underscores() {
        assert_eq!(sanitize_hostname("my.host_name"), "my-host-name");
    }
}
