//! Reachable endpoint derivation from a UDP beacon's packet source.

use std::net::IpAddr;

pub(super) fn from_source(advertised: &str, source: IpAddr) -> String {
    let Ok(mut url) = reqwest::Url::parse(advertised) else {
        return advertised.to_string();
    };
    if url.set_ip_host(source).is_err() {
        return advertised.to_string();
    }
    url.as_str().trim_end_matches('/').to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn packet_source_replaces_an_unreachable_advertised_host() {
        let source = "192.168.1.4".parse().unwrap();
        assert_eq!(
            super::from_source("http://172.17.0.1:4097", source),
            "http://192.168.1.4:4097"
        );
    }
}
