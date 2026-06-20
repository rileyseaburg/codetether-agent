//! Cerebras provider construction.
//!
//! Cerebras exposes an OpenAI-compatible `/v1/chat/completions` API with full
//! tool-calling and structured streaming support. We therefore route Cerebras
//! through the OpenAI-compatible [`OpenAIProvider`](crate::provider::openai::OpenAIProvider)
//! rather than a text-only chat adapter, so agentic tool use and streaming
//! (required by GLM-4.7 and friends) work correctly.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::openai::OpenAIProvider;
use crate::provider::traits::Provider;

const BASE_URL: &str = "https://api.cerebras.ai/v1";
const NAME: &str = "cerebras";

/// Build a Cerebras provider (OpenAI-compatible) from an API key.
pub fn new(api_key: &str, base_url: Option<&str>) -> Result<OpenAIProvider> {
    OpenAIProvider::with_base_url(api_key.to_string(), base_url.unwrap_or(BASE_URL).into(), NAME)
}

/// Build a Cerebras provider as a boxed `Arc<dyn Provider>`.
pub fn provider(api_key: &str, base_url: Option<&str>) -> Result<Arc<dyn Provider>> {
    Ok(Arc::new(new(api_key, base_url)?))
}

/// Default Cerebras OpenAI-compatible API base URL.
pub fn default_base_url() -> &'static str {
    BASE_URL
}
