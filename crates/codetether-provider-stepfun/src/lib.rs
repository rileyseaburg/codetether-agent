//! StepFun chat request conversion, HTTP completion, and streaming.
//!
//! Request, response, transport, catalog, and stream processing have distinct
//! owners. The existing per-HTTP-chunk streaming selection is retained.
use anyhow::Result;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use futures::StreamExt;
mod catalog;
mod construct;
mod debug;
mod provider_impl;
mod request;
mod response;
mod streaming;
mod transport;
mod wire;
use request::ChatRequest;
use wire::*;
const STEPFUN_API_BASE: &str = "https://api.stepfun.ai/v1";
/// StepFun provider using the shared HTTP connection pool.
///
/// # Examples
///
///     let provider = codetether_provider_stepfun::StepFunProvider::new("example-secret".into()).unwrap();
///     assert!(!format!("{provider:?}").contains("example-secret"));
pub struct StepFunProvider {
    api_key: String,
    client: reqwest::Client,
}
