//! Attach reusable SSO metadata to a Bedrock Vault record.
use super::{sso_cache, sso_profile_meta, sso_vault_extra};
use serde_json::Value;
use std::collections::HashMap;

pub(super) fn attach_sso(profile: Option<&str>, extra: &mut HashMap<String, Value>) {
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
