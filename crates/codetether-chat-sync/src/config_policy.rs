//! Resolve scheduling interval and TLS verification policy.
use crate::{
    config_types::*,
    config_vault::vault_bool_val,
    env_helpers::{env_bool, env_non_empty},
};
use codetether_secrets::ProviderSecrets;
pub(super) fn policy(vault: Option<&ProviderSecrets>) -> (u64, bool) {
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
    (iv, tls)
}
