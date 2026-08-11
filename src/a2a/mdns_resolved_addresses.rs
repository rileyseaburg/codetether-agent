//! IPv4 endpoint extraction from scoped mDNS addresses.

use crate::a2a::mdns_addr::order_by_reachability;
use mdns_sd::ScopedIp;
use std::{collections::HashSet, net::Ipv4Addr};

pub(super) fn urls(addresses: &HashSet<ScopedIp>, port: u16, self_port: u16) -> Vec<String> {
    let mut ranked: Vec<(Ipv4Addr, String)> = addresses
        .iter()
        .filter_map(ipv4)
        .filter(|address| !(port == self_port && address.is_loopback()))
        .map(|address| (address, format!("http://{address}:{port}")))
        .collect();
    order_by_reachability(&mut ranked);
    ranked.into_iter().map(|(_, url)| url).collect()
}

fn ipv4(address: &ScopedIp) -> Option<Ipv4Addr> {
    match address {
        ScopedIp::V4(address) => Some(*address.addr()),
        ScopedIp::V6(_) => None,
        _ => None,
    }
}

#[cfg(test)]
#[path = "mdns_resolved_addresses_tests.rs"]
mod tests;
