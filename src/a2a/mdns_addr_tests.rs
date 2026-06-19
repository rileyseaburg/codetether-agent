//! Tests for mDNS address reachability ranking.

use super::{order_by_reachability, reachability_rank};
use std::net::Ipv4Addr;

#[test]
fn lan_ranks_before_docker() {
    let lan = reachability_rank(Ipv4Addr::new(192, 168, 1, 5));
    let docker = reachability_rank(Ipv4Addr::new(172, 19, 0, 1));
    assert!(lan < docker);
}

#[test]
fn order_puts_lan_first() {
    let mut v = vec![
        (Ipv4Addr::new(172, 19, 0, 1), "docker".to_string()),
        (Ipv4Addr::new(192, 168, 1, 5), "lan".to_string()),
    ];
    order_by_reachability(&mut v);
    assert_eq!(v[0].1, "lan");
}
