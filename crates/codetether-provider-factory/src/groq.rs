//! Groq construction using the shared OpenAI-compatible inference transport.
use anyhow::{Result, ensure};
use codetether_provider_core::Provider;
use codetether_provider_openai::{GROQ_BASE_URL, OpenAIProvider};
use std::sync::Arc;
#[cfg(test)]
mod tests;

pub(crate) fn provider(key: &str, base_url: Option<&str>) -> Result<Arc<dyn Provider>> {
    ensure!(!key.trim().is_empty(), "Groq requires a nonempty API key");
    Ok(Arc::new(OpenAIProvider::with_base_url(
        key.into(),
        base_url.unwrap_or(GROQ_BASE_URL).into(),
        "groq",
    )?))
}
