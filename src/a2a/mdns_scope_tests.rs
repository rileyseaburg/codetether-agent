use std::net::{IpAddr, Ipv4Addr};

use super::bounded;

#[test]
fn rejects_unbounded_interface_discovery() {
    assert!(bounded(Vec::new()).is_err());
    assert!(bounded(vec![IpAddr::V4(Ipv4Addr::UNSPECIFIED)]).is_err());
}

#[test]
fn deduplicates_concrete_interfaces() {
    let address = IpAddr::V4(Ipv4Addr::LOCALHOST);
    assert_eq!(bounded(vec![address, address]).unwrap(), vec![address]);
}

#[test]
fn rejects_excessive_interface_counts() {
    let addresses = (1..=9)
        .map(|octet| IpAddr::V4(Ipv4Addr::new(10, 0, 0, octet)))
        .collect();
    assert!(bounded(addresses).is_err());
}
