use std::net::{IpAddr, Ipv4Addr};

use super::concrete;

#[test]
fn prefers_the_public_ip() {
    let listener = IpAddr::V4(Ipv4Addr::LOCALHOST);
    assert_eq!(
        concrete("10.0.0.7", listener),
        vec!["10.0.0.7".parse::<IpAddr>().unwrap()]
    );
}

#[test]
fn rejects_a_loopback_only_listener() {
    let listener = IpAddr::V4(Ipv4Addr::LOCALHOST);
    assert!(concrete("localhost", listener).is_empty());
}

#[test]
fn rejects_unbounded_addresses() {
    let wildcard = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    assert!(concrete("0.0.0.0", wildcard).is_empty());
}
