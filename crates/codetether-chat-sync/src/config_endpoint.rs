//! Resolve endpoint using environment and Vault precedence.
use crate::{
    config_types::normalize_minio_endpoint, config_vault::vault_str, env_helpers::env_non_empty,
};
use codetether_secrets::ProviderSecrets;
pub(super) fn endpoint(vault: Option<&ProviderSecrets>) -> Result<String, String> {
    let ep_raw = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ENDPOINT")
        .or_else(|| {
            vault.as_ref().and_then(|s| {
                s.base_url
                    .clone()
                    .filter(|v| !v.trim().is_empty())
                    .or_else(|| vault_str(s, &["endpoint", "minio_endpoint"]))
            })
        })
        .ok_or("CODETETHER_CHAT_SYNC_MINIO_ENDPOINT required")?;
    let endpoint = normalize_minio_endpoint(&ep_raw);
    Ok(endpoint)
}
