//! Read and normalize the explicitly configured Forgejo principal.
use super::{
    forgejo_agent_identity,
    normalize::{normalize_component, normalize_host},
};
pub(crate) fn configured() -> Option<String> {
    let (host, login, slot) = configured_principal()?;
    forgejo_agent_identity(&host, &login, &slot)
}
pub(crate) fn configured_principal() -> Option<(String, String, String)> {
    let host = std::env::var("CODETETHER_FORGEJO_HOST").ok()?;
    let login = std::env::var("CODETETHER_FORGEJO_LOGIN").ok()?;
    let slot = std::env::var("CODETETHER_AGENT_SLOT").unwrap_or_else(|_| "default".into());
    Some((
        normalize_host(&host)?,
        normalize_component(&login)?,
        normalize_component(&slot)?,
    ))
}
