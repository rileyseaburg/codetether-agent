//! Environment configuration for provider registration.
use crate::Registry;
use std::sync::Arc;

pub(super) fn register_glm5(registry: &mut impl Registry) {
    if registry.contains("glm5") {
        return;
    }
    let Ok(base_url) = std::env::var("GLM5_BASE_URL") else {
        return;
    };
    let key = std::env::var("GLM5_API_KEY").unwrap_or_default();
    let model = std::env::var("GLM5_MODEL").unwrap_or_else(|_| crate::glm5::DEFAULT_MODEL.into());
    match crate::glm5::Glm5Provider::with_model(key, base_url, model) {
        Ok(p) => {
            tracing::info!("Registered glm5 from GLM5_BASE_URL env var");
            registry.register(Arc::new(p));
        }
        Err(e) => tracing::warn!("Failed to init glm5 from env: {e}"),
    }
}
