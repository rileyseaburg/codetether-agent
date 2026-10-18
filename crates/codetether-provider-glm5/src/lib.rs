//! GLM-5 provider for an OpenAI-compatible vLLM endpoint.
//!
//! Construct [Glm5Provider] with endpoint credentials, then use the shared
//! [Provider] trait for buffered or streamed completions. Request encoding,
//! response decoding, transport, and streaming have separate owners.
//!
//! # Endpoint configuration
//! GLM-5 FP8 provider for Vast.ai serverless deployments
//!
//! This provider connects to a self-hosted vLLM endpoint running GLM-5 FP8
//! on Vast.ai serverless infrastructure. The endpoint exposes an OpenAI-compatible
//! chat completions API (vLLM default).
//!
//! ## Model: GLM-5-FP8
//!
//! - **Architecture**: 744B parameter Mixture-of-Experts (MoE)
//! - **Active Parameters**: 40B per forward pass
//! - **Quantization**: FP8 for efficient inference
//! - **Hardware**: 8x A100 SXM4 80GB
//! - **Features**: MTP speculative decoding enabled
//!
//! ## Configuration (Vault)
//!
//! Store under `secret/data/codetether/providers/glm5`:
//! ```json
//! {
//!   "api_key": "<vast-endpoint-api-key>",
//!   "base_url": "https://route.vast.ai/<endpoint-id>/<api-key>/v1",
//!   "extra": {
//!     "model_name": "glm-5-fp8"
//!   }
//! }
//! ```
//!
//! ## Model reference format
//!
//! Use `glm5/glm-5-fp8`, `glm5/glm-5`, or just `glm-5-fp8` as the model string.
//!
//! ## Environment variable fallback
//!
//! - `GLM5_API_KEY` — API key for the Vast.ai endpoint
//! - `GLM5_BASE_URL` — Base URL of the vLLM endpoint (required)
//! - `GLM5_MODEL` — Model name override (default: glm-5-fp8)
//!
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
