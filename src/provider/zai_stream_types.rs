//! Z.AI Server-Sent Event (SSE) wire types for streaming chat completions.
//!
//! These deserialize the `data:` frames emitted by the Z.AI streaming
//! `/chat/completions` endpoint. They are kept separate from the provider
//! logic in [`crate::provider::zai`] so the wire schema has a single home.

use super::ZaiUsage;
use serde::Deserialize;
use serde_json::Value;

/// A single SSE frame from the Z.AI streaming endpoint.
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiStreamResponse {
    #[serde(default)]
    pub(crate) choices: Vec<ZaiStreamChoice>,
    #[serde(default)]
    pub(crate) usage: Option<ZaiUsage>,
}

/// One choice within an SSE frame.
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiStreamChoice {
    pub(crate) delta: ZaiStreamDelta,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}

/// Incremental delta payload carried by an SSE choice.
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiStreamDelta {
    #[serde(default)]
    pub(crate) content: Option<String>,
    #[serde(default)]
    pub(crate) reasoning_content: Option<String>,
    #[serde(default)]
    pub(crate) tool_calls: Option<Vec<ZaiStreamToolCall>>,
}

/// A streamed tool-call fragment.
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiStreamToolCall {
    #[serde(default)]
    pub(crate) index: Option<usize>,
    #[serde(default)]
    pub(crate) id: Option<String>,
    pub(crate) function: Option<ZaiStreamFunction>,
}

/// The function portion of a streamed tool-call fragment.
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiStreamFunction {
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) arguments: Option<Value>,
}
