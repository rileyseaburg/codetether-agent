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
fn falls_back_to_a_resolved_listener() {
    let listener = IpAddr::V4(Ipv4Addr::LOCALHOST);
    assert_eq!(concrete("localhost", listener), vec![listener]);
}

#[test]
fn rejects_unbounded_addresses() {
    let wildcard = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    assert!(concrete("0.0.0.0", wildcard).is_empty());
}
