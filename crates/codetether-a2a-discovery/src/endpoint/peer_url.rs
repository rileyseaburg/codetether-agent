//! Normalization and candidate expansion for explicit A2A peer URLs.

use anyhow::{Context, Result};
use reqwest::Url;
use std::collections::HashSet;

/// Normalize a peer seed into an HTTP(S) base URL without trailing slashes.
///
/// # Arguments
/// * `url` - Peer address, optionally without a scheme.
/// # Returns
/// The parsed base URL, with HTTP supplied when no HTTP(S) scheme is present.
/// # Errors
/// Returns an error for an empty address or a URL that cannot be parsed.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::endpoint::normalize_base_url;
/// assert_eq!(normalize_base_url(" localhost:5000/ ").unwrap(), "http://localhost:5000");
/// ```
pub fn normalize_base_url(url: &str) -> Result<String> {
    let trimmed = url.trim();
    anyhow::ensure!(!trimmed.is_empty(), "URL cannot be empty");
    let normalized = match trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        true => trimmed.to_string(),
        false => format!("http://{trimmed}"),
    };
    let parsed = Url::parse(&normalized).with_context(|| format!("Invalid URL: {normalized}"))?;
    Ok(parsed.as_str().trim_end_matches('/').to_string())
}

/// Normalize and deduplicate peer seeds, excluding this node's address.
///
/// # Arguments
/// * `raw` - Configured peer addresses; invalid addresses are skipped.
/// * `self_url` - This node's normalized base URL.
/// # Returns
/// Unique peer base URLs in unspecified order.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::endpoint::collect_peers;
/// assert!(collect_peers(&["localhost:5000".into()], "http://localhost:5000").is_empty());
/// ```
pub fn collect_peers(raw: &[String], self_url: &str) -> Vec<String> {
    raw.iter()
        .filter_map(|peer| normalize_base_url(peer).ok())
        .filter(|peer| peer != self_url.trim_end_matches('/'))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

/// Expand a seed into base and compatible A2A endpoint candidates.
///
/// # Arguments
/// * `seed` - Normalized peer base URL.
/// # Returns
/// The original seed followed by its `/a2a` variant, unless already suffixed.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::endpoint::peer_candidates;
/// assert_eq!(peer_candidates("http://host/a2a"), ["http://host/a2a"]);
/// ```
pub fn peer_candidates(seed: &str) -> Vec<String> {
    match seed.ends_with("/a2a") {
        true => vec![seed.to_string()],
        false => vec![seed.to_string(), format!("{seed}/a2a")],
    }
}

#[cfg(test)]
#[path = "peer_url_tests.rs"]
mod tests;
