use crate::provider::init_dispatch;
use crate::provider::init_dispatch_impl;
use crate::provider::registry::ProviderRegistry;
use crate::secrets::ProviderSecrets;
use anyhow::Result;

pub(super) fn provider(
    registry: &mut ProviderRegistry,
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
        init_dispatch_impl::codex::dispatch_vault(&provider_id, &secrets)
    } else {
        init_dispatch::dispatch(&provider_id, &secrets)
    };
    if let Some(provider) = provider {
        registry.register(provider);
    }
}
