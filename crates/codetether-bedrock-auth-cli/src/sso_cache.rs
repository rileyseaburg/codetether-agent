//! AWS SSO cache lookup for refresh-token material.

use super::sso_profile_meta::SsoProfileMeta;
use anyhow::{Context, Result};
use serde::Deserialize;

/// Relevant fields from `~/.aws/sso/cache/*.json`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SsoTokenCache {
    pub start_url: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub registration_expires_at: Option<String>,
    pub refresh_token: Option<String>,
}

/// Find a cache entry with a refresh token for `meta`.
pub(super) fn find(meta: &SsoProfileMeta) -> Result<Option<SsoTokenCache>> {
    for path in super::sso_cache_files::json_files()? {
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let Ok(cache) = serde_json::from_str::<SsoTokenCache>(&text) else {
            continue;
        };
        if matches_profile(&cache, meta) && has_refresh_token(&cache) {
            return Ok(Some(cache));
        }
    }
    Ok(None)
}

pub(super) fn matches_profile(cache: &SsoTokenCache, meta: &SsoProfileMeta) -> bool {
    cache
        .start_url
        .as_deref()
        .is_some_and(|url| normalize(url) == normalize(&meta.start_url))
}

fn has_refresh_token(cache: &SsoTokenCache) -> bool {
    cache
        .refresh_token
        .as_deref()
        .is_some_and(|token| !token.is_empty())
}

fn normalize(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}
