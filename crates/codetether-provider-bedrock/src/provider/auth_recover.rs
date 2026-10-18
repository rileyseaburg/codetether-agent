//! Shared 401/403 recovery for every Bedrock request path.
//!
//! When Bedrock rejects a bearer token, the SSO/STS session behind it has
//! expired. Recovery is two-tiered so all live sessions converge on one key:
//! 1. **Adopt** — if Vault already holds a *different* `api_key` (another
//!    process refreshed it), swap that in without minting a new one.
//! 2. **Re-mint** — otherwise run the silent SSO refresh and save to Vault.
//!
//! Callers retry the original request exactly once when this returns `true`.

use crate::provider::{BedrockProvider, sso_refresh};
pub(crate) use crate::recovery::is_auth_failure;
use codetether_secrets as secrets;

/// Try to replace the provider's bearer token. Returns `true` when a new
/// token was installed and the caller should retry. SigV4 providers never
/// recover here (their credentials are not Vault-managed API keys).
pub(crate) async fn recover(provider: &BedrockProvider) -> bool {
    let Some(current) = provider.auth.current_bearer() else {
        return false;
    };
    if let Some(newer) = newer_vault_key(&current).await {
        provider.auth.set_bearer(newer);
        tracing::info!(target: "codetether_agent::provider::bedrock::exports::auth_recover",
            provider = "bedrock",
            "adopted refreshed Bedrock key from Vault"
        );
        return true;
    }
    match sso_refresh::refresh_now(true).await {
        Ok(refreshed) => {
            provider.auth.set_bearer(refreshed.token);
            tracing::info!(target: "codetether_agent::provider::bedrock::exports::auth_recover", provider = "bedrock", "refreshed bearer token mid-session");
            true
        }
        Err(e) => {
            tracing::warn!(target: "codetether_agent::provider::bedrock::exports::auth_recover", provider = "bedrock", error = %e, "mid-session refresh failed");
            false
        }
    }
}

async fn newer_vault_key(current: &str) -> Option<String> {
    let stored = secrets::get_provider_secrets("bedrock").await?;
    adoptable(stored.api_key.as_deref(), current)
}

/// A stored key is worth adopting only if it is non-empty and differs from
/// the token that was just rejected.
fn adoptable(stored: Option<&str>, current: &str) -> Option<String> {
    stored
        .filter(|key| !key.is_empty() && *key != current)
        .map(str::to_string)
}

#[cfg(test)]
#[path = "auth_recover_tests.rs"]
mod tests;
