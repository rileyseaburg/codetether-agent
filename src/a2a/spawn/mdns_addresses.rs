//! Concrete interface selection for bounded mDNS discovery.

use std::net::IpAddr;

pub(super) fn concrete(public_host: &str, listener: IpAddr) -> Vec<IpAddr> {
    public_host
        .parse()
        .ok()
        .filter(|address: &IpAddr| !address.is_unspecified())
        .or_else(|| (!listener.is_unspecified()).then_some(listener))
        .into_iter()
        .collect()
}

#[cfg(test)]
#[path = "mdns_addresses_tests.rs"]
mod tests;
