//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_glm5(key: &str, secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::glm5;
    let url = secrets.base_url.clone().unwrap_or_else(|| {
        std::env::var("GLM5_BASE_URL").unwrap_or_else(|_| "https://localhost:18000/v1".into())
    });
    let model = secrets
        .extra
        .get("model_name")
        .and_then(|v| v.as_str())
        .unwrap_or(glm5::DEFAULT_MODEL)
        .to_string();
    match glm5::Glm5Provider::with_model(key.into(), url, model) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init glm5: {e}");
            None
        }
    }
}
