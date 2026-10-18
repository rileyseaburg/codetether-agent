//! OpenAI-compatible request wire contracts.
use serde::Deserialize;
use crate::{OpenAiRequestMessage, OpenAiRequestTool};

#[derive(Debug, Deserialize)]
/// OpenAI-compatible OpenAiChatCompletionRequest wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiChatCompletionRequest>(r#"{"model":"model","messages":[]}"#).unwrap();
/// let _ = value;
/// ```
pub struct OpenAiChatCompletionRequest {
    /// model wire field.
    pub model: String,
    /// messages wire field.
    pub messages: Vec<OpenAiRequestMessage>,
    #[serde(default)]
    /// tools wire field.
    pub tools: Vec<OpenAiRequestTool>,
    /// temperature wire field.
    pub temperature: Option<f32>,
    /// top_p wire field.
    pub top_p: Option<f32>,
    /// max_tokens wire field.
    pub max_tokens: Option<usize>,
    /// max_completion_tokens wire field.
    pub max_completion_tokens: Option<usize>,
    /// stop wire field.
    pub stop: Option<OpenAiStop>,
    /// stream wire field.
    pub stream: Option<bool>,
    /// reasoning_effort wire field.
    pub reasoning_effort: Option<String>,
    /// service_tier wire field.
    pub service_tier: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// OpenAI-compatible OpenAiStop wire representation.
/// # Examples
/// ```
/// let value = serde_json::from_str::<codetether_openai_wire::OpenAiStop>(r#""stop""#).unwrap();
/// let _ = value;
/// ```
pub enum OpenAiStop {
    /// One stop string.
    Single(String),
    /// Multiple stop strings.
    Multiple(Vec<String>),
}
