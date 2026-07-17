//! Normalization and candidate expansion for explicit A2A peer URLs.

use anyhow::{Context, Result};
use reqwest::Url;
use std::collections::HashSet;

pub(super) fn normalize_base_url(url: &str) -> Result<String> {
    let trimmed = url.trim();
    anyhow::ensure!(!trimmed.is_empty(), "URL cannot be empty");
    let normalized = match trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        true => trimmed.to_string(),
        false => format!("http://{trimmed}"),
    };
    let parsed = Url::parse(&normalized).with_context(|| format!("Invalid URL: {normalized}"))?;
    Ok(parsed.as_str().trim_end_matches('/').to_string())
}

pub(super) fn collect_peers(raw: &[String], self_url: &str) -> Vec<String> {
    raw.iter()
        .filter_map(|peer| normalize_base_url(peer).ok())
        .filter(|peer| peer != self_url.trim_end_matches('/'))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub(super) fn peer_candidates(seed: &str) -> Vec<String> {
    match seed.ends_with("/a2a") {
        true => vec![seed.to_string()],
        false => vec![seed.to_string(), format!("{seed}/a2a")],
    }
}

#[cfg(test)]
#[path = "peer_url_tests.rs"]
mod tests;
