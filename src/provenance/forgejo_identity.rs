//! Canonical Forgejo principal-to-agent identity derivation.

use sha2::{Digest, Sha256};

/// Derives the routable identity for a Forgejo account and agent slot.
///
/// # Arguments
///
/// * `host` — Forgejo host name, optionally prefixed by `http(s)://`.
/// * `login` — Verified Forgejo signer login.
/// * `slot` — Stable logical agent slot, such as `default` or `reviewer`.
///
/// # Returns
///
/// A deterministic identity, or `None` when an input is unsafe.
///
/// # Examples
///
/// ```
/// use codetether_agent::provenance::forgejo_agent_identity;
///
/// let first = forgejo_agent_identity("forge.example", "Alice", "default").unwrap();
/// let second = forgejo_agent_identity("https://forge.example", "alice", "default").unwrap();
/// assert_eq!(first, second);
/// assert!(first.starts_with("ctforgejo_"));
/// ```
pub fn forgejo_agent_identity(host: &str, login: &str, slot: &str) -> Option<String> {
    let host = normalize_host(host)?;
    let login = normalize_component(login)?;
    let slot = normalize_component(slot)?;
    let digest = Sha256::digest(format!("{host}\n{login}\n{slot}"));
    Some(format!("ctforgejo_{}", &hex::encode(digest)[..40]))
}

pub(super) fn configured() -> Option<String> {
    let (host, login, slot) = configured_principal()?;
    forgejo_agent_identity(&host, &login, &slot)
}

pub(super) fn configured_principal() -> Option<(String, String, String)> {
    let host = std::env::var("CODETETHER_FORGEJO_HOST").ok()?;
    let login = std::env::var("CODETETHER_FORGEJO_LOGIN").ok()?;
    let slot = std::env::var("CODETETHER_AGENT_SLOT").unwrap_or_else(|_| "default".into());
    Some((
        normalize_host(&host)?,
        normalize_component(&login)?,
        normalize_component(&slot)?,
    ))
}

fn normalize_host(value: &str) -> Option<String> {
    let value = value.trim().to_ascii_lowercase();
    let value = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"))
        .unwrap_or(&value);
    let host = value.split('/').next().unwrap_or_default();
    valid(host, ".-:").then(|| host.to_string())
}

fn normalize_component(value: &str) -> Option<String> {
    let value = value.trim().to_ascii_lowercase();
    valid(&value, "._-").then_some(value)
}

fn valid(value: &str, extra: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphanumeric())
        && value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || extra.contains(ch))
}

#[cfg(test)]
#[path = "forgejo_identity_tests.rs"]
mod tests;
