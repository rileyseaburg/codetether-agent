//! Ownership and cleanup of a registered mDNS service.

use mdns_sd::ServiceDaemon;
use std::sync::Arc;

/// Handle that keeps an advertised mDNS service alive.
///
/// Dropping the handle unregisters the service and stops its daemon.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::a2a::mdns::announce_and_browse;
/// # use std::net::{IpAddr, Ipv4Addr};
/// # let (tx, _) = tokio::sync::mpsc::channel(8);
/// let handle = announce_and_browse(
///     "agent",
///     8000,
///     vec![IpAddr::V4(Ipv4Addr::LOCALHOST)],
///     "token",
///     tx,
/// )?;
/// handle.shutdown();
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct MdnsHandle {
    daemon: Arc<ServiceDaemon>,
    fullname: String,
}

impl MdnsHandle {
    pub(super) fn new(daemon: Arc<ServiceDaemon>, fullname: String) -> Self {
        Self { daemon, fullname }
    }

    /// Immediately unregisters the service and stops discovery.
    pub fn shutdown(self) {
        drop(self);
    }
}

impl Drop for MdnsHandle {
    fn drop(&mut self) {
        let _ = self.daemon.unregister(&self.fullname);
        let _ = self.daemon.shutdown();
    }
}
