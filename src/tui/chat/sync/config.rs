//! Configuration parsing from env vars and Vault.

use super::config_types::*;
use super::env_helpers::{env_bool, env_non_empty, env_non_placeholder};

fn vault_str(s: &crate::secrets::ProviderSecrets, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| {
        s.extra
            .get(*k)
            .and_then(|v| v.as_str())
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    })
}

fn vault_bool_val(s: &crate::secrets::ProviderSecrets, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|k| {
        s.extra
            .get(*k)
            .and_then(|v| v.as_str())
            .and_then(|v| match v {
                "true" | "1" | "yes" => Some(true),
                "false" | "0" | "no" => Some(false),
                _ => None,
            })
    })
}

/// Parse chat sync config. Returns `None` if disabled, `Err` if required but misconfigured.
pub async fn parse_chat_sync_config(required: bool) -> Result<Option<ChatSyncConfig>, String> {
    if !env_bool("CODETETHER_CHAT_SYNC_ENABLED").unwrap_or(false) {
        if required {
            return Err("CODETETHER_CHAT_SYNC_ENABLED must be true".into());
        }
        return Ok(None);
    }
    let vault = crate::secrets::get_provider_secrets("chat-sync-minio").await;
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
    let sk = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY")
        .or_else(|| {
            vault
                .as_ref()
                .and_then(|s| vault_str(s, &["secret_key", "password", "secret"]))
        })
        .ok_or("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY required")?;
    let bucket = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_BUCKET")
        .or_else(|| vault.as_ref().and_then(|s| vault_str(s, &["bucket"])))
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_BUCKET.into());
    let prefix = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_PREFIX")
        .or_else(|| vault.as_ref().and_then(|s| vault_str(s, &["prefix"])))
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_PREFIX.into())
        .trim_matches('/')
        .to_string();
    let iv = env_non_empty("CODETETHER_CHAT_SYNC_INTERVAL_SECS")
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(CHAT_SYNC_DEFAULT_INTERVAL_SECS)
        .clamp(1, CHAT_SYNC_MAX_INTERVAL_SECS);
    let tls = env_bool("CODETETHER_CHAT_SYNC_MINIO_INSECURE_SKIP_TLS_VERIFY")
        .or_else(|| {
            vault
                .as_ref()
                .and_then(|s| vault_bool_val(s, &["insecure_skip_tls_verify"]))
        })
        .unwrap_or(false);
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
