//! Streaming choices, reasoning deltas, and incremental tool-call wire types.
pub(super) use super::sse_usage::{GroqMetadata, SseUsage};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(super) struct SseResponse {
    #[serde(default)]
    pub(super) choices: Vec<SseChoice>,
    #[serde(default)]
    pub(super) usage: Option<SseUsage>,
    #[serde(default)]
    pub(super) x_groq: Option<GroqMetadata>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SseChoice {
    pub(super) delta: SseDelta,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct SseDelta {
    #[serde(default)]
    pub(super) content: Option<String>,
    #[serde(default, alias = "reasoning")]
    pub(super) reasoning_content: Option<String>,
    #[serde(default)]
    pub(super) tool_calls: Option<Vec<SseToolCall>>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SseToolCall {
    #[serde(default)]
    pub(super) index: usize,
    #[serde(default)]
    pub(super) id: Option<String>,
    #[serde(default)]
    pub(super) function: Option<SseFunction>,
}

#[derive(Debug, Deserialize)]
pub(super) struct SseFunction {
    #[serde(default)]
    pub(super) name: Option<String>,
    #[serde(default)]
    pub(super) arguments: Option<Value>,
}
