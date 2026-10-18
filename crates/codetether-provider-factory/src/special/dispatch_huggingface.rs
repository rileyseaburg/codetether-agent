//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_huggingface(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::openai;
    let url = secrets.base_url.as_ref()?;
    match openai::OpenAIProvider::with_base_url_optional_key(
        secrets.api_key.clone(),
        url.clone(),
        "huggingface",
    ) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init huggingface: {e}");
            None
        }
    }
}
