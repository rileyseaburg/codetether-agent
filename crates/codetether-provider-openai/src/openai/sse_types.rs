//! SSE wire types for OpenAI-compatible streaming with reasoning support.
//!
//! The `async-openai` typed stream drops vendor reasoning fields
//! (`reasoning_content` / `reasoning`). Cerebras reasoning-class models
//! (e.g. `gemma-4-31b`) can emit a turn that is reasoning-only before any
//! `content`, which the typed path renders as an empty assistant message and
//! surfaces as "stream ended without assistant content". These types capture
//! the reasoning delta so such turns yield non-empty thinking content.

use serde::Deserialize;
use serde_json::Value;

/// A single SSE `data:` frame.
#[derive(Debug, Deserialize)]
pub(super) struct SseResponse {
    #[serde(default)]
    pub(super) choices: Vec<SseChoice>,
    #[serde(default)]
    pub(super) usage: Option<SseUsage>,
}

/// One choice within an SSE frame.
#[derive(Debug, Deserialize)]
pub(super) struct SseChoice {
    pub(super) delta: SseDelta,
}

/// Incremental delta payload.
#[derive(Debug, Deserialize, Default)]
pub(super) struct SseDelta {
    #[serde(default)]
    pub(super) content: Option<String>,
    #[serde(default, alias = "reasoning")]
    pub(super) reasoning_content: Option<String>,
    #[serde(default)]
    pub(super) tool_calls: Option<Vec<SseToolCall>>,
}

/// A streamed tool-call fragment.
#[derive(Debug, Deserialize)]
pub(super) struct SseToolCall {
    #[serde(default)]
    pub(super) index: usize,
    #[serde(default)]
    pub(super) id: Option<String>,
    #[serde(default)]
    pub(super) function: Option<SseFunction>,
}

/// Function portion of a tool-call fragment.
#[derive(Debug, Deserialize)]
pub(super) struct SseFunction {
    #[serde(default)]
    pub(super) name: Option<String>,
    #[serde(default)]
    pub(super) arguments: Option<Value>,
}

/// Usage block carried by a terminal SSE frame.
#[derive(Debug, Deserialize)]
pub(super) struct SseUsage {
    #[serde(default)]
    pub(super) prompt_tokens: usize,
    #[serde(default)]
    pub(super) completion_tokens: usize,
    #[serde(default)]
    pub(super) total_tokens: usize,
}
