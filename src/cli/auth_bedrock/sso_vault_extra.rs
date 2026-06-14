//! Build Vault extra fields for Bedrock SSO refresh metadata.

use super::{sso_cache::SsoTokenCache, sso_profile_meta::SsoProfileMeta};
use serde_json::Value;
use std::collections::HashMap;

/// Insert non-empty AWS SSO refresh metadata into a provider secret map.
pub(super) fn insert(
    extra: &mut HashMap<String, Value>,
    meta: &SsoProfileMeta,
    cache: &SsoTokenCache,
) {
    put(extra, "sso_profile", &meta.profile);
    put(extra, "sso_start_url", &meta.start_url);
    put(extra, "sso_region", &meta.sso_region);
    put_opt(extra, "sso_account_id", meta.account_id.as_deref());
    put_opt(extra, "sso_role_name", meta.role_name.as_deref());
    put_opt(extra, "sso_access_token", cache.access_token.as_deref());
    put_opt(extra, "sso_refresh_token", cache.refresh_token.as_deref());
    put_opt(extra, "sso_client_id", cache.client_id.as_deref());
    put_opt(extra, "sso_client_secret", cache.client_secret.as_deref());
    put_opt(extra, "sso_expires_at", cache.expires_at.as_deref());
    put_opt(
        extra,
        "sso_registration_expires_at",
        cache.registration_expires_at.as_deref(),
    );
}

fn put(extra: &mut HashMap<String, Value>, key: &str, value: &str) {
    extra.insert(key.to_string(), Value::String(value.to_string()));
}

fn put_opt(extra: &mut HashMap<String, Value>, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.is_empty()) {
        put(extra, key, value);
    }
}
