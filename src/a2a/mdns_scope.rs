//! Resource bounds for mDNS interface selection.

use std::net::IpAddr;

use anyhow::{Result, bail};

const MAX_INTERFACES: usize = 8;

/// Validate and bound the concrete interfaces assigned to one mDNS daemon.
pub(super) fn bounded(mut addresses: Vec<IpAddr>) -> Result<Vec<IpAddr>> {
    addresses.retain(|address| !address.is_unspecified());
    addresses.sort_unstable();
    addresses.dedup();
    if addresses.is_empty() {
        bail!("mDNS requires a concrete interface address")
    }
    if addresses.len() > MAX_INTERFACES {
        bail!("mDNS interface limit exceeded ({MAX_INTERFACES})")
    }
    Ok(addresses)
}

#[cfg(test)]
#[path = "mdns_scope_tests.rs"]
mod tests;
