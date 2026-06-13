//! Output/persistence for `codetether auth bedrock` tokens.

use super::BedrockAuthArgs;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Print the minted token and optionally persist it to Vault.
pub(super) async fn emit(args: &BedrockAuthArgs, region: &str, token: &str) -> Result<()> {
    if args.raw {
        println!("{token}");
    } else {
        println!(
            "Short-term Bedrock API key (region {region}, <= {}s):",
            args.expires_secs
        );
        println!();
        println!("  export AWS_BEARER_TOKEN_BEDROCK={token}");
        println!();
        println!(
            "Note: tokens signed with SSO/STS session credentials expire when the session does."
        );
    }
    if args.save {
        save_to_vault(region, token).await?;
    }
    Ok(())
}

/// Store the token as the `bedrock` provider api_key in Vault.
async fn save_to_vault(region: &str, token: &str) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN, \
             or omit --save and use the printed token directly."
        );
    }
    let mut extra = HashMap::new();
    extra.insert(
        "region".to_string(),
        serde_json::Value::String(region.to_string()),
    );
    let provider_secrets = ProviderSecrets {
        api_key: Some(token.to_string()),
        base_url: None,
        organization: None,
        headers: None,
        extra,
    };
    secrets::set_provider_secrets("bedrock", &provider_secrets)
        .await
        .context("Failed to store Bedrock token in Vault")?;
    println!("Saved short-term Bedrock API key to Vault provider 'bedrock'.");
    Ok(())
}
