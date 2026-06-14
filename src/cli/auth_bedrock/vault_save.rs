//! Vault persistence for minted Bedrock API keys.

use super::{BedrockAuthArgs, vault_extra};
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};

/// Store the token as the `bedrock` provider api_key in Vault.
pub(super) async fn save(
    args: &BedrockAuthArgs,
    region: &str,
    token: &str,
    profile: Option<&str>,
) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN, \
             or omit --save and use the printed token directly."
        );
    }
    let extra = vault_extra::fields(args, region, profile);
    let saved_sso_refresh = extra.contains_key("sso_refresh_token");
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
    if saved_sso_refresh {
        println!("Saved AWS SSO refresh metadata with the Bedrock provider record.");
    }
    Ok(())
}
