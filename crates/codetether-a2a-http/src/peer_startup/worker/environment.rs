//! Environment settings for optional worker-local peers.
pub(super) fn worker_a2a_mdns_enabled() -> bool {
    std::env::var("CODETETHER_WORKER_A2A_MDNS")
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
}

pub(super) fn worker_a2a_port() -> u16 {
    std::env::var("CODETETHER_WORKER_A2A_PORT")
        .ok()
        .and_then(|value| value.trim().parse::<u16>().ok())
        .unwrap_or(8081)
}

pub(super) fn worker_a2a_peer_seeds() -> Vec<String> {
    ["CODETETHER_WORKER_A2A_PEERS", "CODETETHER_A2A_PEERS"]
        .into_iter()
        .filter_map(|name| std::env::var(name).ok())
        .flat_map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|peer| !peer.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .collect()
}
