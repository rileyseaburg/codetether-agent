//! Worker-side A2A peer configuration helpers.

use crate::cli::A2aArgs;

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

pub(super) fn worker_public_url_for_port(args: &A2aArgs, port: u16) -> Option<String> {
    let configured = args
        .public_url
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    reqwest::Url::parse(configured)
        .map(|mut url| {
            let _ = url.set_port(Some(port));
            url.as_str().trim_end_matches('/').to_string()
        })
        .ok()
        .or_else(|| Some(configured.trim_end_matches('/').to_string()))
}
