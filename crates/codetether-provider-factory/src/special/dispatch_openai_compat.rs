//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_openai_compat(
    key: &str,
    secrets: &ProviderSecrets,
    pid: &str,
) -> Option<Arc<dyn Provider>> {
    use crate::openai;
    if let Some(url) = &secrets.base_url {
        match openai::OpenAIProvider::with_base_url(key.into(), url.clone(), pid) {
            Ok(p) => return Some(Arc::new(p)),
            Err(e) => {
                tracing::warn!("Failed to init {pid}: {e}");
                return None;
            }
        }
    }
    if pid == "openai" {
        match openai::OpenAIProvider::new(key.into()) {
            Ok(p) => return Some(Arc::new(p)),
            Err(e) => {
                tracing::warn!("Failed to init openai: {e}");
                return None;
            }
        }
    }
    if pid == "novita" {
        let url = "https://api.novita.ai/openai/v1".to_string();
        match openai::OpenAIProvider::with_base_url(key.into(), url, pid) {
            Ok(p) => return Some(Arc::new(p)),
            Err(e) => {
                tracing::warn!("Failed to init {pid}: {e}");
                return None;
            }
        }
    }
    tracing::warn!("Provider {pid} has no built-in base_url; set base_url in Vault");
    None
}
