//! Google Gemini OpenAI-compatible chat transport.
//!
//! The provider owns credential state and delegates request conversion, model
//! metadata, HTTP execution, and response interpretation to focused modules.
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
mod debug;
mod provider_impl;
mod request;
mod response;
mod transport;
const GOOGLE_OPENAI_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/openai";
/// Google Gemini provider using the shared HTTP connection pool.
///
/// # Examples
///
///     let provider = codetether_provider_google::GoogleProvider::new("example-secret".into()).unwrap();
///     assert!(!format!("{provider:?}").contains("example-secret"));
pub struct GoogleProvider {
    client: Client,
    api_key: String,
}
