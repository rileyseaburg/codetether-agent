//! Resolve chat-sync configuration without starting a worker.
use crate::{config_types::*, env_helpers::env_bool};
/// Parse chat sync config, returning None when disabled.
/// # Errors
/// Returns missing-configuration errors when sync is enabled or required.
/// Resolve optional archive synchronization settings.
/// # Arguments
/// Whether configuration is required.
/// # Returns
/// Configuration when enabled, or None when optional and disabled.
/// # Errors
/// Returns missing or disabled required configuration errors.
/// # Examples
/// ```text
/// Configure the archive destination, then use parse_chat_sync_config through the chat-sync API.
/// ```
pub async fn parse_chat_sync_config(required: bool) -> Result<Option<ChatSyncConfig>, String> {
    if !env_bool("CODETETHER_CHAT_SYNC_ENABLED").unwrap_or(false) {
        if required {
            return Err("CODETETHER_CHAT_SYNC_ENABLED must be true".into());
        }
        return Ok(None);
    }
    let vault = codetether_secrets::get_provider_secrets("chat-sync-minio").await;
    let endpoint = crate::config_endpoint::endpoint(vault.as_ref())?;
    let ak = crate::config_access::access_key(vault.as_ref())?;
    let sk = crate::config_secret::secret_key(vault.as_ref())?;
    let (bucket, prefix) = crate::config_location::location(vault.as_ref());
    let (iv, tls) = crate::config_policy::policy(vault.as_ref());
    Ok(Some(ChatSyncConfig {
        endpoint: endpoint.clone(),
        fallback_endpoint: minio_fallback_endpoint(&endpoint),
        access_key: ak,
        secret_key: sk,
        bucket,
        prefix,
        interval_secs: iv,
        ignore_cert_check: tls,
    }))
}
