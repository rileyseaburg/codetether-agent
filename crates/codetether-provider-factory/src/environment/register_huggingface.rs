//! Environment configuration for provider registration.
use crate::Registry;
use std::sync::Arc;

pub(super) fn register_huggingface(registry: &mut impl Registry) {
    if registry.contains("huggingface") {
        return;
    }
    let url = std::env::var("HF_BASE_URL")
        .ok()
        .or_else(|| std::env::var("HUGGINGFACE_BASE_URL").ok())
        .or_else(|| std::env::var("HUGGINGFACE_ENDPOINT").ok());
    let Some(base_url) = url else { return };
    let key = std::env::var("HF_TOKEN")
        .ok()
        .or_else(|| std::env::var("HUGGINGFACE_API_KEY").ok())
        .filter(|v| !v.trim().is_empty());
    match crate::openai::OpenAIProvider::with_base_url_optional_key(key, base_url, "huggingface") {
        Ok(p) => {
            tracing::info!("Registered huggingface from HF_BASE_URL env var");
            registry.register(Arc::new(p));
        }
        Err(e) => tracing::warn!("Failed to init huggingface from env: {e}"),
    }
}
