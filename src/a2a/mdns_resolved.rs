//! Conversion of resolved mDNS services into discovery peers.

use super::{DiscoveredPeer, SERVICE_TYPE, resolved_addresses};
use mdns_sd::ResolvedService;

pub(super) fn peer(
    info: &ResolvedService,
    self_fullname: &str,
    self_port: u16,
) -> Option<DiscoveredPeer> {
    let fullname = info.get_fullname();
    tracing::debug!(
        %fullname,
        port = info.get_port(),
        addrs = ?info.get_addresses(),
        "mDNS service resolved"
    );
    if fullname == self_fullname {
        return None;
    }
    let urls = resolved_addresses::urls(info.get_addresses(), info.get_port(), self_port);
    if urls.is_empty() {
        return None;
    }
    Some(DiscoveredPeer {
        urls,
        instance_name: instance_name(fullname),
        token: info.get_property_val_str("auth").map(ToString::to_string),
    })
}

fn instance_name(fullname: &str) -> String {
    fullname
        .strip_suffix(SERVICE_TYPE)
        .unwrap_or(fullname)
        .trim_end_matches('.')
        .to_string()
}
