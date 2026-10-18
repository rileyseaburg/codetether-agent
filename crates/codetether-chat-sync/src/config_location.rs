//! Resolve bucket and archive prefix.
use crate::{config_types::*, config_vault::vault_str, env_helpers::env_non_empty};
use codetether_secrets::ProviderSecrets;
pub(super) fn location(vault: Option<&ProviderSecrets>) -> (String, String) {
    let bucket = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_BUCKET")
        .or_else(|| vault.as_ref().and_then(|s| vault_str(s, &["bucket"])))
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_BUCKET.into());
    let prefix = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_PREFIX")
        .or_else(|| vault.as_ref().and_then(|s| vault_str(s, &["prefix"])))
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_PREFIX.into())
        .trim_matches('/')
        .to_string();
    (bucket, prefix)
}
