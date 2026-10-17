//! Derive a stable routable identity from a normalized Forgejo principal.
use super::normalize::{normalize_component, normalize_host};
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
/// use codetether_provenance::forgejo_agent_identity;
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
