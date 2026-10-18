//! Store imported cookies and check their availability through Vault.
use anyhow::{Context, Result};
use codetether_secrets::{self as secrets, ProviderSecrets};
use serde_json::Value;
use std::collections::HashMap;

pub(super) async fn save(provider_id: &str, extra: HashMap<String, Value>) -> Result<bool> {
    let provider_secrets = ProviderSecrets {
        api_key: None,
        base_url: None,
        organization: None,
        headers: None,
        extra,
    };

    secrets::set_provider_secrets(provider_id, &provider_secrets)
        .await
        .with_context(|| format!("Failed to store {} cookies in Vault", provider_id))?;
    let can_read_back = secrets::get_provider_secrets(provider_id)
        .await
        .map(|saved| saved.extra.contains_key("cookies"))
        .unwrap_or(false);

    Ok(can_read_back)
}
