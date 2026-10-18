//! Resolve access key using environment and Vault precedence.
use crate::{config_vault::vault_str, env_helpers::env_non_placeholder};
use codetether_secrets::ProviderSecrets;
pub(super) fn access_key(vault: Option<&ProviderSecrets>) -> Result<String, String> {
    let ak = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY")
        .or_else(|| {
            vault.as_ref().and_then(|s| {
                vault_str(s, &["access_key", "api_key"]).or_else(|| {
                    s.api_key
                        .as_ref()
                        .map(|v: &String| v.trim().to_string())
                        .filter(|v: &String| !v.is_empty())
                })
            })
        })
        .ok_or("CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY required")?;
    Ok(ak)
}
