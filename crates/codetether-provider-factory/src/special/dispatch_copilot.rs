//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_copilot<H: crate::Host>(
    key: &str,
    secrets: &ProviderSecrets,
    name: &str,
) -> Option<Arc<dyn Provider>> {
    let result = if let Some(url) = &secrets.base_url {
        H::copilot_with_base_url(key.into(), url.clone(), name)
    } else {
        H::copilot_new(key.into())
    };
    match result {
        Ok(p) => Some(p),
        Err(e) => {
            tracing::warn!("Failed to init {name}: {e}");
            None
        }
    }
}
