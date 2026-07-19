//! Process-wide identity shared by provenance and A2A transports.

use std::sync::OnceLock;

static IDENTITY: OnceLock<String> = OnceLock::new();

/// Returns the explicitly configured or registered runtime agent identity.
///
/// # Returns
///
/// The stable identity selected for this process, or `None` when neither a
/// configured nor persistent identity is available.
///
/// # Examples
///
/// ```no_run
/// use codetether_agent::provenance::runtime_agent_identity;
///
/// let identity = runtime_agent_identity();
/// assert!(identity.is_none() || identity.unwrap().len() > 0);
/// ```
pub fn runtime_agent_identity() -> Option<String> {
    super::forgejo_identity::configured()
        .or_else(configured_identity)
        .or_else(|| IDENTITY.get().cloned())
        .or_else(persisted_identity)
}

/// Registers `advertised_name` as the fallback identity for this process.
///
/// # Arguments
///
/// * `advertised_name` — Worker name used only when no configured or persisted
///   identity exists.
///
/// # Returns
///
/// The stable identity selected for the process.
///
/// # Examples
///
/// ```
/// use codetether_agent::provenance::ensure_runtime_agent_identity;
///
/// assert!(!ensure_runtime_agent_identity("worker").is_empty());
/// ```
pub fn ensure_runtime_agent_identity(advertised_name: &str) -> String {
    runtime_agent_identity().unwrap_or_else(|| bind_runtime_agent_identity(advertised_name))
}

pub(crate) fn bind_runtime_agent_identity(identity: &str) -> String {
    if let Some(configured) = super::forgejo_identity::configured().or_else(configured_identity) {
        return configured;
    }
    let identity = identity.trim().to_string();
    let _ = IDENTITY.set(identity.clone());
    IDENTITY.get().cloned().unwrap_or(identity)
}

fn configured_identity() -> Option<String> {
    std::env::var("CODETETHER_AGENT_IDENTITY_ID")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn persisted_identity() -> Option<String> {
    let identity = super::runtime_identity_store::load_or_create()?;
    let _ = IDENTITY.set(identity.clone());
    IDENTITY.get().cloned().or(Some(identity))
}
