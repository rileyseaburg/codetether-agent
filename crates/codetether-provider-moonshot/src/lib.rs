//! Moonshot chat completion, K2 request policy, and response conversion.
//!
//! MoonshotProvider uses the shared HTTP pool and provider contracts. Request,
//! response, catalog, and transport modules each own their protocol concern.
use anyhow::{Context, Result};
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
mod catalog;
mod construct;
mod provider_impl;
mod request;
mod response;
mod transport;
/// Direct Moonshot provider using the shared HTTP connection pool.
///
/// # Examples
///
///     let provider = codetether_provider_moonshot::MoonshotProvider::new(String::new());
///     assert!(provider.is_ok());
pub struct MoonshotProvider {
    client: Client,
    api_key: String,
    base_url: String,
}
