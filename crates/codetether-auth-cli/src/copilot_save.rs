//! Vault persistence for public or enterprise Copilot tokens.
use anyhow::{Context, Result};
use codetether_secrets::{self as secrets, ProviderSecrets};
use std::collections::HashMap;

pub(super) async fn save(
    provider_id: &str,
    enterprise_domain: Option<String>,
    token: String,
) -> Result<()> {
    let mut extra = HashMap::new();
    if let Some(enterprise_url) = enterprise_domain {
        extra.insert(
            "enterpriseUrl".to_string(),
            serde_json::Value::String(enterprise_url),
        );
    }

    let provider_secrets = ProviderSecrets {
        api_key: Some(token),
        base_url: None,
        organization: None,
        headers: None,
        extra,
    };

    secrets::set_provider_secrets(provider_id, &provider_secrets)
        .await
        .with_context(|| format!("Failed to store {} auth token in Vault", provider_id))?;

    println!("Saved {} credentials to HashiCorp Vault.", provider_id);
    Ok(())
}
