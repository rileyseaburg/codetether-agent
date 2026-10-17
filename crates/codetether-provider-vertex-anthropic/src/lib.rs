//! Vertex Anthropic protocol, retries, and streaming.
//!
//! Authentication is supplied by the shared Google service-account crate.
//! Request, response, transport, catalog, and streaming responsibilities remain separate.
use anyhow::{Context, Result};
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;
mod construct;
mod debug;
mod image_content;
mod provider_impl;
mod request;
mod response;
mod streaming;
mod transport;
#[path = "vertex_anthropic_catalog.rs"]
pub mod vertex_anthropic_catalog;
#[path = "vertex_anthropic_model.rs"]
pub mod vertex_anthropic_model;
#[path = "vertex_anthropic_rows.rs"]
pub mod vertex_anthropic_rows;
use response::wire::*;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRIES: u32 = 3;
const VERTEX_REGION: &str = "us-east5";
const VERTEX_ANTHROPIC_VERSION: &str = "vertex-2023-10-16";
/// Claude provider using Vertex endpoints and shared service-account auth.
///
/// # Examples
///
///     let result = codetether_provider_vertex_anthropic::VertexAnthropicProvider::new("{}", None);
///     assert!(result.is_err());
pub struct VertexAnthropicProvider {
    client: Client,
    project_id: String,
    base_url: String,
    auth: codetether_google_auth::ServiceAccountAuth,
}

#[cfg(test)]
mod tests;
