//! Gemini Web provider orchestration over shared protocol and transport crates.
//!
//! [GeminiWebProvider] owns client construction, cookies, completion assembly,
//! and the validated tool-protocol retry. Transport and parsing live in their
//! dedicated workspace crates.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_gemini_web_protocol::MODELS;
use codetether_gemini_web_transport::SessionTokens;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition,
};
use reqwest::Client;
use serde_json::Value;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
mod state;
pub use state::GeminiWebProvider;
mod complete;
mod methods;
mod model;
mod prompt;
mod provider_impl;
mod response_text;
mod stream;
mod stream_output;
#[cfg(test)]
mod tests;
mod tool_calls;
mod tool_validation;
mod usage;
