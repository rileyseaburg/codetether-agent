//! GLM-5 provider for an OpenAI-compatible vLLM endpoint.
//!
//! Construct [Glm5Provider] with endpoint credentials, then use the shared
//! [Provider] trait for buffered or streamed completions. Request encoding,
//! response decoding, transport, and streaming have separate owners.
use anyhow::{Context, Result};
use async_trait::async_trait;
/// Default model name served by the endpoint.
pub use codetether_provider_core::model_identity::glm5::DEFAULT_MODEL;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use futures::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
mod catalog;
mod construct;
mod debug;
mod model;
mod provider;
mod request;
mod response;
mod streaming;
mod transport;
mod wire;
use wire::*;
/// GLM-5 FP8 provider for an OpenAI-compatible vLLM endpoint.
///
/// # Examples
/// ```no_run
/// let provider = codetether_provider_glm5::Glm5Provider::new(
///     "endpoint-key".into(), "https://example.invalid/v1".into())?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct Glm5Provider {
    client: Client,
    api_key: String,
    base_url: String,
    /// The actual model name to send in API requests (e.g. "glm-5-fp8").
    model_name: String,
}
#[cfg(test)]
mod tests;
