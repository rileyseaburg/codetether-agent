//! Apply one Vault secret result while retaining Codex refresh identity.

use crate::Registry;
use anyhow::Result;
use codetether_secrets::ProviderSecrets;

pub(super) fn provider<H: crate::Host>(
    registry: &mut impl Registry,
    provider_id: String,
    secrets: Result<Option<ProviderSecrets>>,
) {
    let secrets = match secrets {
        Ok(Some(secrets)) => secrets,
        Ok(None) => return,
        Err(error) => {
            tracing::warn!(provider = %provider_id, %error, "vault fetch failed; skipping");
            return;
        }
    };
    let provider = if matches!(provider_id.as_str(), "openai-codex" | "codex" | "chatgpt") {
        crate::codex::dispatch_vault(&provider_id, &secrets)
    } else {
        crate::dispatch::<H>(&provider_id, &secrets)
    };
    if let Some(provider) = provider {
        registry.register(provider);
    }
}
