//! Ordering of mDNS-resolved peer addresses by reachability likelihood.
//!
//! A multi-homed peer advertises every interface IP — LAN, VPN, and
//! container bridges (`docker0`, `br-*`, `veth*`). The intake loop tries
//! the resolved URLs in order, so a docker-bridge IP like `172.19.0.1`
//! sorted first wastes a connection attempt on an address no off-host peer
//! can reach. This module ranks IPv4 addresses so the most broadly routable
//! ones are tried first, without dropping any (a bridge IP is occasionally
//! the only route on a single host).

use std::net::Ipv4Addr;

/// Reachability rank for `addr`; lower is tried first.
///
/// Ordinary LAN/public addresses rank best; the Docker default-bridge range
/// (`172.16.0.0/12`) and link-local (`169.254.0.0/16`) rank worst; loopback
/// sits between (reachable same-host only).
///
/// # Examples
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use codetether_agent::a2a::mdns_addr::reachability_rank;
///
/// let lan = reachability_rank(Ipv4Addr::new(192, 168, 50, 101));
/// let docker = reachability_rank(Ipv4Addr::new(172, 19, 0, 1));
/// let link_local = reachability_rank(Ipv4Addr::new(169, 254, 1, 1));
/// assert!(lan < docker);
/// assert!(lan < link_local);
/// ```
pub fn reachability_rank(addr: Ipv4Addr) -> u8 {
    let [a, b, ..] = addr.octets();
    if addr.is_link_local() {
        3
    } else if a == 172 && (16..=31).contains(&b) {
        // Docker/container default bridge range (172.16.0.0/12).
        3
    } else if addr.is_loopback() {
        2
    } else {
        0
    }
}

/// Stably sort `urls` paired with their source IPv4 by [`reachability_rank`].
///
/// Equal-rank addresses keep their original (resolver) order.
///
/// # Examples
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use codetether_agent::a2a::mdns_addr::order_by_reachability;
///
/// let mut v = vec![
///     (Ipv4Addr::new(172, 19, 0, 1), "docker".to_string()),
///     (Ipv4Addr::new(192, 168, 1, 5), "lan".to_string()),
/// ];
/// order_by_reachability(&mut v);
/// assert_eq!(v[0].1, "lan");
/// ```
pub fn order_by_reachability(urls: &mut [(Ipv4Addr, String)]) {
    urls.sort_by_key(|(addr, _)| reachability_rank(*addr));
}

#[cfg(test)]
#[path = "mdns_addr_tests.rs"]
mod mdns_addr_tests;
