//! Resolve secret key using environment and Vault precedence.
use crate::{config_vault::vault_str, env_helpers::env_non_placeholder};
use codetether_secrets::ProviderSecrets;
pub(super) fn secret_key(vault: Option<&ProviderSecrets>) -> Result<String, String> {
    let sk = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY")
        .or_else(|| {
            vault
                .as_ref()
                .and_then(|s| vault_str(s, &["secret_key", "password", "secret"]))
        })
        .ok_or("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY required")?;
    Ok(sk)
}
