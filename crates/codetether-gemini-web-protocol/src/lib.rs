//! Gemini Web prompt, tool protocol, response decoding, and usage estimates.
//!
//! [GeminiWebProtocol] contains stateless wire helpers. [prompt] renders
//! history; [tool_validation] checks emitted calls; [response_text] selects
//! answers; [stream_output] emits provider-neutral chunks.
use regex::Regex;
use serde_json::{Value, json};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
/// Stateless Gemini Web wire encoding and diagnostics.
///
/// # Examples
/// ```
/// let body = codetether_gemini_web_protocol::GeminiWebProtocol::build_freq("hello");
/// assert!(serde_json::from_str::<serde_json::Value>(&body).is_ok());
/// ```
pub struct GeminiWebProtocol;
pub mod prompt;
#[cfg(test)]
mod prompt_tests;
pub mod response_text;
pub mod stream_output;
pub mod tool_calls;
pub mod tool_validation;
pub mod usage;
mod wire;

mod model;
mod model_catalog;
pub use model_catalog::MODELS;
