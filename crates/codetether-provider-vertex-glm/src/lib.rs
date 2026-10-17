//! Vertex AI GLM completion provider.
//!
//! Construct [VertexGlmProvider] with service-account JSON, then call the
//! [Provider] completion methods. Authentication is owned by the shared Google
//! auth crate; request, response, transport, and streaming have separate modules.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use futures::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;
mod catalog;
mod construct;
mod debug;
mod provider;
mod request;
mod response;
mod streaming;
mod transport;
mod wire;
use wire::*;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRIES: u32 = 3;
const VERTEX_ENDPOINT: &str = "aiplatform.googleapis.com";
const VERTEX_REGION: &str = "global";
/// GLM completions served through Vertex AI with cached service-account authentication.
///
/// # Examples
/// ```no_run
/// # fn example(json: &str) -> anyhow::Result<()> {
/// let provider = codetether_provider_vertex_glm::VertexGlmProvider::new(json, None)?;
/// # Ok(())
/// # }
/// ```
pub struct VertexGlmProvider {
    client: Client,
    project_id: String,
    base_url: String,
    auth: codetether_google_auth::ServiceAccountAuth,
}
#[cfg(test)]
mod tests;
