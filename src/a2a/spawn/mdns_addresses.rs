//! Concrete interface selection for bounded mDNS discovery.

use std::net::IpAddr;

pub(super) fn concrete(public_host: &str, listener: IpAddr) -> Vec<IpAddr> {
    public_host
        .parse()
        .ok()
        .filter(is_publishable)
        .or_else(|| is_publishable(&listener).then_some(listener))
        .into_iter()
        .collect()
}

fn is_publishable(address: &IpAddr) -> bool {
    !address.is_unspecified() && !address.is_loopback()
}

#[cfg(test)]
#[path = "mdns_addresses_tests.rs"]
mod tests;
