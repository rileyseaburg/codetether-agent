//! Selection of the concrete address advertised by wildcard listeners.

use std::net::{IpAddr, Ipv4Addr};

pub(super) fn first_lan_ipv4() -> Option<String> {
    if_addrs::get_if_addrs()
        .ok()?
        .into_iter()
        .filter_map(|interface| match interface.ip() {
            IpAddr::V4(address) if is_candidate(address) => Some((interface.name, address)),
            IpAddr::V4(_) | IpAddr::V6(_) => None,
        })
        .min_by_key(|(name, address)| {
            (
                is_container_interface(name),
                crate::a2a::mdns_addr::reachability_rank(*address),
            )
        })
        .map(|(_, address)| address.to_string())
}

fn is_candidate(address: Ipv4Addr) -> bool {
    !address.is_loopback() && !address.is_link_local() && !address.is_unspecified()
}

fn is_container_interface(name: &str) -> bool {
    ["br-", "cni", "docker", "flannel", "veth"]
        .iter()
        .any(|prefix| name.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    #[test]
    fn recognizes_common_container_interfaces() {
        assert!(super::is_container_interface("docker0"));
        assert!(super::is_container_interface("br-1234"));
        assert!(!super::is_container_interface("ens18"));
    }
}
