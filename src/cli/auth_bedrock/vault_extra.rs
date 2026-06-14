//! Vault extra-field builder for Bedrock auth records.

use super::{BedrockAuthArgs, sso_cache, sso_profile_meta, sso_vault_extra};
use crate::provider::bedrock::token_gen;
use serde_json::{Value, json};
use std::collections::HashMap;

/// Build provider-specific metadata for the Bedrock Vault secret.
pub(super) fn fields(
    args: &BedrockAuthArgs,
    region: &str,
    profile: Option<&str>,
) -> HashMap<String, Value> {
    let lifetime = args.expires_secs.min(token_gen::DEFAULT_EXPIRES_SECS);
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(lifetime as i64);
    let mut extra = HashMap::from([
        ("region".to_string(), Value::String(region.to_string())),
        (
            "api_key_expires_at".to_string(),
            json!(expires_at.timestamp()),
        ),
        (
            "api_key_expires_at_rfc3339".to_string(),
            json!(expires_at.to_rfc3339()),
        ),
    ]);
    attach_sso(profile, &mut extra);
    extra
}

fn attach_sso(profile: Option<&str>, extra: &mut HashMap<String, Value>) {
    let Some(profile) = profile else { return };
    let meta = match sso_profile_meta::resolve(profile) {
        Ok(Some(meta)) => meta,
        Ok(None) => return,
        Err(error) => {
            tracing::warn!(profile, %error, "Failed to inspect AWS SSO profile");
            return;
        }
    };
    match sso_cache::find(&meta) {
        Ok(Some(cache)) => sso_vault_extra::insert(extra, &meta, &cache),
        Ok(None) => {}
        Err(error) => tracing::warn!(profile, %error, "Failed to inspect AWS SSO cache"),
    }
}
