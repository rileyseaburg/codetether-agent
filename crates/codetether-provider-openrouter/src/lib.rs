//! OpenRouter completion, reasoning configuration, and streaming.
//!
//! Public capability and runtime configuration modules retain their API.
//! Provider state delegates history, discovery, and execution to focused modules.
use anyhow::{Context, Result};
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
mod catalog;
mod completion;
mod construct;
#[path = "error_detail.rs"]
mod error_detail;
mod errors;
mod history;
#[path = "model_capabilities.rs"]
pub mod model_capabilities;
mod provider_impl;
#[path = "reasoning_levels.rs"]
pub mod reasoning_levels;
#[path = "reasoning_request.rs"]
mod reasoning_request;
#[path = "request_body.rs"]
mod request_body;
mod response;
#[path = "runtime_config.rs"]
pub mod runtime_config;
#[path = "schema.rs"]
mod schema;
#[path = "stream.rs"]
mod stream;
use response::wire::*;
use response::wire_error::*;
/// OpenRouter provider with custom timeouts and shared TCP socket options.
///
/// # Examples
///
///     let provider = codetether_provider_openrouter::OpenRouterProvider::new(String::new());
///     assert!(provider.is_ok());
pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

#[cfg(test)]
mod tests;
