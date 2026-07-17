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
//!   - `auth=<process-scoped collaboration capability>`
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
//! Code in `spawn.rs` mirrors this by advertising the concrete IP used in
//! the agent card, including for wildcard listeners. This avoids publishing
//! container bridges and other host-only interfaces as peer candidates.

use anyhow::{Context, Result};
use mdns_sd::{IfKind, ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::mpsc;

#[path = "mdns_hostname.rs"]
mod hostname;
pub use hostname::sanitize_hostname;
#[path = "mdns_scope.rs"]
mod scope;

/// mDNS service type used by all CodeTether A2A peers.
pub const SERVICE_TYPE: &str = "_codetether-a2a._tcp.local.";
/// Handle to a registered mDNS service. Drop or call [`Self::shutdown`] to
/// unregister and stop the daemon.
///
/// Owns the resources created when advertising the local agent over mDNS. The
/// handle keeps the underlying [`ServiceDaemon`] alive and records the service's
/// full instance name so it can be unregistered cleanly when discovery should
/// stop.
///
/// # Lifecycle
///
/// Call [`Self::shutdown`] when the service should be removed immediately. If
/// the handle is dropped without an explicit shutdown call, its drop behavior is
/// expected to perform the same cleanup path so the advertised service does not
/// remain registered longer than the owning process intends.
///
/// # Fields
///
/// * `daemon` - Shared mDNS service daemon used to unregister the advertised
///   service and stop background mDNS work.
/// * `fullname` - Fully qualified mDNS service instance name registered with
///   the daemon.
pub struct MdnsHandle {
    daemon: Arc<ServiceDaemon>,
    fullname: String,
}
impl MdnsHandle {
    /// Best-effort unregister and shut the daemon down.
    ///
    /// Consumes the handle and attempts to remove the advertised service from
    /// the local mDNS daemon before shutting the daemon down. This is useful for
    /// deterministic cleanup when the caller knows the advertised agent should
    /// no longer be discoverable.
    ///
    /// # Side effects
    ///
    /// Calls `unregister` for the service identified by `self.fullname`, then
    /// calls `shutdown` on the shared [`ServiceDaemon`]. Peers may continue to
    /// see cached mDNS records until their TTL expires.
    ///
    /// # Errors
    ///
    /// Errors from both daemon operations are intentionally ignored. Shutdown is
    /// best-effort because the daemon may already have stopped or the service may
    /// already have been unregistered, especially during repeated shutdown
    /// signals or drop-time cleanup.
    pub fn shutdown(self) {
        // Calling unregister is best-effort; the daemon may already be
        // gone if shutdown_signal arrived twice.
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}

impl Drop for MdnsHandle {
    /// Unregisters the advertised mDNS service and shuts down the service daemon
    /// when the handle leaves scope.
    ///
    /// This provides best-effort cleanup for callers that do not explicitly call
    /// [`MdnsHandle::shutdown`]. Both cleanup operations intentionally ignore
    /// their return values because `drop` cannot report errors and cleanup may
    /// race with prior explicit shutdown or daemon termination.
    ///
    /// # Side effects
    ///
    /// Removes the service identified by `self.fullname` from the local mDNS
    /// daemon and then requests daemon shutdown. Network peers may stop seeing
    /// the service after mDNS cache expiry rather than immediately.
    fn drop(&mut self) {
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}
/// A peer discovered over mDNS — every reachable URL the resolver saw,
/// so the existing A2A discovery flow can try them in order.
///
/// Represents one advertised A2A peer service instance resolved from mDNS.
/// A single service instance can map to multiple network addresses when the
/// host has more than one interface, such as Ethernet, Wi-Fi, VPN, container
/// bridge, or loopback-adjacent addresses. The discovery layer keeps all
/// candidate URLs so the caller can attempt them in resolver order and select
/// the first endpoint that returns a usable agent card.
///
/// # Invariants
///
/// * `urls` contains fully formed HTTP endpoint URLs for the resolved service
///   port.
/// * `instance_name` is the mDNS service instance name and is expected to match
///   the peer's advertised card name.
/// * URL ordering is resolver-dependent and should not be treated as a stable
///   preference across networks or process runs.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DiscoveredPeer {
    /// All `http://<addr>:<port>` URLs reported for this peer's service
    /// record. In multi-homed environments mDNS may resolve several IPs
    /// (LAN, VPN, docker bridge, etc.); the intake loop tries them in
    /// order until one responds with a valid agent card. The first entry
    /// is whichever address mdns-sd reported first (network-dependent).
    pub urls: Vec<String>,
    /// Service instance name reported by mDNS (== card name).
    pub instance_name: String,
    /// Per-process bearer capability advertised by a first-party peer.
    pub token: Option<String>,
}

/// Announce ourselves on mDNS and start a browse loop that emits every
/// resolved peer (other than ourselves) onto `peer_tx`.
///
/// Returns once the service is registered and the browse loop is running
/// in a background tokio task. The returned [`MdnsHandle`] keeps the
/// daemon alive — drop it on shutdown.
///
/// Registers the local A2A JSON-RPC endpoint as an mDNS service and begins
/// browsing for other endpoints using the same service type. Resolved peers are
/// converted into [`DiscoveredPeer`] values containing all usable IPv4 HTTP URLs
/// reported by the resolver, then forwarded through `peer_tx` for the existing
/// A2A intake flow to probe.
///
/// # Parameters
///
/// * `instance_name` - Human-readable service instance name advertised in mDNS
///   and stored in TXT records. It is also sanitized into the `.local.` hostname
///   used by the service registration.
/// * `bind_port` - TCP port where the local A2A HTTP server is listening. This
///   port is advertised to peers and used to filter loopback self-observations.
/// * `bound_addrs` - Concrete IP addresses accepted by the HTTP listener. The
///   list must be non-empty and is bounded to prevent unrestrained socket growth.
/// * `collaboration_token` - Process-scoped bearer capability that lets another
///   discovered CodeTether peer call the A2A endpoint without sharing the
///   control-plane token.
/// * `peer_tx` - Async channel used by the blocking mDNS browse task to forward
///   resolved peers into the caller's discovery pipeline.
///
/// # Returns
///
/// Returns an [`MdnsHandle`] after the local service has been registered and the
/// background browse task has been spawned. Keeping the handle alive keeps the
/// underlying [`ServiceDaemon`] alive; dropping it or calling
/// [`MdnsHandle::shutdown`] unregisters the service.
///
/// # Errors
///
/// Returns an error if the mDNS daemon cannot be started, the service
/// description cannot be constructed, the local service cannot be registered, or
/// browsing for `SERVICE_TYPE` cannot be started.
///
/// # Side effects
///
/// Starts an mDNS daemon, publishes TXT records describing the local A2A peer,
/// registers the service on the local network, and spawns a blocking background
/// task that receives mDNS events and forwards resolved peers to `peer_tx`.
pub fn announce_and_browse(
    instance_name: &str,
    bind_port: u16,
    bound_addrs: Vec<IpAddr>,
    collaboration_token: &str,
    peer_tx: mpsc::Sender<DiscoveredPeer>,
) -> Result<MdnsHandle> {
    let bound_addrs = scope::bounded(bound_addrs)?;
    let daemon = ServiceDaemon::new().context("Failed to start mDNS daemon")?;
    daemon.disable_interface(IfKind::All)?;
    daemon.enable_interface(bound_addrs.clone())?;
    let daemon = Arc::new(daemon);

    // Properties surfaced in TXT records so peers can sanity-check us.
    let mut props: HashMap<String, String> = HashMap::new();
    props.insert("name".to_string(), instance_name.to_string());
    props.insert("path".to_string(), "/".to_string());
    props.insert("protocol".to_string(), "a2a-jsonrpc".to_string());
    props.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    props.insert("auth".to_string(), collaboration_token.to_string());

    // mdns-sd's hostname must end with `.local.` and be unique per service
    // instance on the LAN. We derive it from the instance name.
    let mdns_hostname = format!("{}.local.", sanitize_hostname(instance_name));

    let service = ServiceInfo::new(
        SERVICE_TYPE,
        instance_name,
        &mdns_hostname,
        bound_addrs.as_slice(),
        bind_port,
        Some(props),
    )
    .context("Failed to construct mDNS ServiceInfo")?;

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
                    let mut ranked: Vec<(std::net::Ipv4Addr, String)> = info
                        .get_addresses()
                        .iter()
                        .filter_map(|a| match a {
                            IpAddr::V4(v4) => Some(*v4),
                            IpAddr::V6(_) => None,
                        })
                        .filter(|v4| !(port == self_port && v4.is_loopback()))
                        .map(|v4| (v4, format!("http://{v4}:{port}")))
                        .collect();
                    crate::a2a::mdns_addr::order_by_reachability(&mut ranked);
                    let urls: Vec<String> = ranked.into_iter().map(|(_, url)| url).collect();
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
                        token: info.get_property_val_str("auth").map(ToString::to_string),
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
