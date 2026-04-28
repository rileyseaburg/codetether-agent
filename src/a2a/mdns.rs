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
//! - **Zero-config.** Peers on the same LAN find each other on launch
//!   without any flags. Same-host setups also work *as long as the agents
//!   bind a real network interface* (e.g. `--hostname 0.0.0.0`); see the
//!   loopback note below.
//! - **Standard.** Same protocol as Bonjour/Avahi/Chromecast/AirPlay.
//!
//! # Loopback caveat
//!
//! `mdns-sd` is willing to use the loopback interface, but on Linux the
//! `lo` interface lacks the `MULTICAST` flag by default — multicast
//! traffic does not traverse it, so two agents both bound to `127.0.0.1`
//! will NOT find each other via mDNS even though they're on the same
//! host. The reliable single-host pattern is to bind `0.0.0.0`: each
//! agent advertises on the real LAN interface, and multicast loopback
//! through that interface (controlled by `IP_MULTICAST_LOOP`, default on)
//! delivers the announcement to other same-host listeners.
//!
//! Code in `spawn.rs` mirrors this by passing only the bound IPs that are
//! actually reachable: loopback addrs alone when `--hostname 127.0.0.1`,
//! and an empty list (which `enable_addr_auto` then expands to all
//! detected interfaces) when `--hostname 0.0.0.0` is requested.

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

/// A peer discovered over mDNS — every reachable URL the resolver saw,
/// so the existing A2A discovery flow can try them in order.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoveredPeer {
    /// All `http://<addr>:<port>` URLs reported for this peer's service
    /// record. In multi-homed environments mDNS may resolve several IPs
    /// (LAN, VPN, docker bridge, etc.); the intake loop tries them in
    /// order until one responds with a valid agent card. The first entry
    /// is whichever address mdns-sd reported first (network-dependent).
    pub urls: Vec<String>,
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

    // If the caller passed concrete bound addrs (e.g. for `--hostname
    // 127.0.0.1` they pass loopback only) we use exactly those — that
    // matches what the HTTP server will actually accept.
    //
    // If the caller passed an empty list (i.e. they bound `0.0.0.0` and
    // genuinely want all interfaces advertised), we ask mdns-sd to
    // auto-detect interface addrs via `enable_addr_auto`. We must still
    // pass at least one address to ServiceInfo::new for it to construct,
    // so use loopback as a placeholder — the auto-detect will replace
    // it with real interface IPs.
    let auto_detect = bound_addrs.is_empty();
    let addrs: Vec<IpAddr> = if auto_detect {
        vec!["127.0.0.1".parse().unwrap()]
    } else {
        bound_addrs.clone()
    };

    let mut service = ServiceInfo::new(
        SERVICE_TYPE,
        instance_name,
        &mdns_hostname,
        addrs.as_slice(),
        bind_port,
        Some(props),
    )
    .context("Failed to construct mDNS ServiceInfo")?;
    if auto_detect {
        service = service.enable_addr_auto();
    }

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
                    // Collect every reachable IPv4 the resolver knows about.
                    // The intake loop will try them in order — useful when
                    // the same agent is reachable on a LAN IP, a VPN IP,
                    // and a docker bridge IP, where the "right" one varies
                    // by caller's network position.
                    let urls: Vec<String> = info
                        .get_addresses()
                        .iter()
                        .filter(|a| a.is_ipv4())
                        .filter(|a| !(port == self_port && a.is_loopback()))
                        .map(|a| format!("http://{a}:{port}"))
                        .collect();
                    if urls.is_empty() {
                        continue;
                    }
                    let instance = info_fullname
                        .strip_suffix(SERVICE_TYPE)
                        .unwrap_or(&info_fullname)
                        .trim_end_matches('.')
                        .to_string();
                    let peer = DiscoveredPeer {
                        urls,
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

/// Produce a valid DNS label: ascii alnum + `-` only, lowercase, no
/// leading/trailing hyphen, ≤ 63 chars (RFC 1035 §2.3.1). An empty
/// result falls back to "agent" so the caller never gets an invalid
/// hostname (which would silently break `ServiceInfo::new`).
fn sanitize_hostname(input: &str) -> String {
    const MAX_LABEL: usize = 63;
    let mut s: String = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else if c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect();
    if s.len() > MAX_LABEL {
        s.truncate(MAX_LABEL);
    }
    let trimmed = s.trim_matches('-');
    if trimmed.is_empty() {
        "agent".to_string()
    } else {
        trimmed.to_string()
    }
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

    #[test]
    fn sanitize_hostname_lowercases() {
        assert_eq!(sanitize_hostname("Alice-Host"), "alice-host");
    }

    #[test]
    fn sanitize_hostname_trims_leading_and_trailing_dashes() {
        assert_eq!(sanitize_hostname(".alice."), "alice");
        assert_eq!(sanitize_hostname("---bob---"), "bob");
    }

    #[test]
    fn sanitize_hostname_truncates_to_63_chars() {
        let long = "a".repeat(100);
        let out = sanitize_hostname(&long);
        assert_eq!(out.len(), 63);
        assert!(out.chars().all(|c| c == 'a'));
    }

    #[test]
    fn sanitize_hostname_falls_back_when_all_dashes() {
        assert_eq!(sanitize_hostname("..."), "agent");
        assert_eq!(sanitize_hostname(""), "agent");
        assert_eq!(sanitize_hostname("---"), "agent");
    }
}
