//! Decode Bedrock Converse response content and service errors.
//!
//! [parse_converse_response] preserves content order, tool calls, token usage,
//! and reasoning signatures. [BedrockError] represents service error payloads.
use crate::reasoning::ReasoningContentBlock;
use anyhow::{Context, Result};
use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, Usage,
};
use serde::Deserialize;
use serde_json::Value;
mod content;
mod error;
mod finish;
mod parse;
mod wire;
pub use error::BedrockError;
pub use parse::parse_converse_response;
use wire::*;
#[cfg(test)]
#[path = "response/tests.rs"]
mod tests;
