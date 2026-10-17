use super::urls;
use mdns_sd::{InterfaceId, ScopedIp, ScopedIpV4};
use std::{collections::HashSet, net::Ipv4Addr};

#[test]
fn extracts_ipv4_from_scoped_address() {
    let address = Ipv4Addr::new(192, 168, 1, 10);
    let scoped = ScopedIpV4::new(address, InterfaceId::default());
    let addresses = HashSet::from([ScopedIp::V4(scoped)]);

    assert_eq!(urls(&addresses, 9000, 8000), ["http://192.168.1.10:9000"]);
}

#[test]
fn filters_own_loopback_endpoint() {
    let scoped = ScopedIpV4::new(Ipv4Addr::LOCALHOST, InterfaceId::default());
    let addresses = HashSet::from([ScopedIp::V4(scoped)]);

    assert!(urls(&addresses, 8000, 8000).is_empty());
}
