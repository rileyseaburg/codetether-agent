//! Concrete interface selection for bounded mDNS discovery.

use std::net::IpAddr;

/// Choose a concrete address for bounded mDNS publication.
///
/// # Arguments
/// * `public_host` - Advertised host, preferred when it is a publishable IP.
/// * `listener` - Bound listener IP, used as fallback.
/// # Returns
/// At most one address; unspecified and loopback addresses are excluded.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::endpoint::concrete;
/// let wildcard = "0.0.0.0".parse().unwrap();
/// assert!(concrete("localhost", wildcard).is_empty());
/// ```
pub fn concrete(public_host: &str, listener: IpAddr) -> Vec<IpAddr> {
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
