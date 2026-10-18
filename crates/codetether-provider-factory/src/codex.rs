//! API-key and OAuth construction for Codex provider secrets.
use codetether_provider_codex::OpenAiCodexProvider;
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

/// Construct Codex from API-key or OAuth credentials without a Vault identity.
///
/// # Arguments
/// * `secrets` — Provider credentials.
/// # Returns
/// A configured provider, or `None` when usable credentials are absent.
pub fn dispatch(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    build(secrets, None)
}

/// Construct Codex with its Vault identity for credential refresh.
///
/// # Arguments
/// * `provider_id` — Vault provider identity.
/// * `secrets` — Provider credentials.
/// # Returns
/// A configured provider, or `None` when usable credentials are absent.
pub fn dispatch_vault(provider_id: &str, secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    build(secrets, Some(provider_id))
}

fn build(secrets: &ProviderSecrets, vault_id: Option<&str>) -> Option<Arc<dyn Provider>> {
    if let Some(key) = secrets.api_key.as_deref().filter(|key| !key.is_empty()) {
        return Some(Arc::new(OpenAiCodexProvider::from_api_key(key.into())));
    }
    let credentials = credentials(secrets)?;
    let provider = match vault_id {
        Some(id) => OpenAiCodexProvider::from_vault_credentials(id, credentials),
        None => OpenAiCodexProvider::from_credentials(credentials),
    };
    Some(Arc::new(provider))
}

pub use codetether_codex_auth::lifecycle::credentials;
