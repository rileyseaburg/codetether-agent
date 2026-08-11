//! Routing for events emitted by the mDNS service browser.

use super::{DiscoveredPeer, resolved};
use mdns_sd::ServiceEvent;
use tokio::sync::mpsc;

pub(super) fn keep_browsing(
    event: ServiceEvent,
    self_fullname: &str,
    self_port: u16,
    peer_tx: &mpsc::Sender<DiscoveredPeer>,
) -> bool {
    match event {
        ServiceEvent::SearchStarted(service) => {
            tracing::debug!(%service, "mDNS search started");
            true
        }
        ServiceEvent::ServiceFound(service, fullname) => {
            tracing::debug!(%service, %fullname, "mDNS service found");
            true
        }
        ServiceEvent::ServiceResolved(info) => resolved::peer(&info, self_fullname, self_port)
            .is_none_or(|peer| peer_tx.blocking_send(peer).is_ok()),
        ServiceEvent::ServiceRemoved(service, fullname) => {
            tracing::debug!(%service, %fullname, "mDNS service removed");
            true
        }
        ServiceEvent::SearchStopped(service) => {
            tracing::debug!(%service, "mDNS search stopped");
            true
        }
        unknown => {
            tracing::debug!(event = ?unknown, "Unknown mDNS service event");
            true
        }
    }
}
