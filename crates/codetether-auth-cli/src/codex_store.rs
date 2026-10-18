//! Persist Codex credentials after optional API-key exchange.
use anyhow::{Context, Result};
use codetether_provider_codex::OAuthCredentials;
use codetether_secrets::{self as secrets, ProviderSecrets};

pub(super) async fn store_codex_credentials(credentials: OAuthCredentials) -> Result<()> {
    let chatgpt_account_id = crate::codex_account::id(&credentials);

    let (api_key, expected_token_exchange_fallback) =
        crate::codex_exchange::key(&credentials).await;

    let extra = crate::codex_metadata::fields(&credentials, &chatgpt_account_id);

    let provider_secrets = ProviderSecrets {
        api_key: api_key.clone(),
        base_url: None,
        organization: chatgpt_account_id.clone(),
        headers: None,
        extra,
    };

    secrets::set_provider_secrets("openai-codex", &provider_secrets)
        .await
        .context("Failed to store openai-codex OAuth credentials in Vault")?;

    crate::codex_report::print(
        &credentials,
        api_key,
        expected_token_exchange_fallback,
        chatgpt_account_id,
    );
    Ok(())
}
