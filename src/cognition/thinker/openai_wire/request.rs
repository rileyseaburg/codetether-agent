//! OpenAI-compatible chat request types for the HTTP thinker backend.

use serde::Serialize;

/// Chat-completions request body.
#[derive(Debug, Serialize)]
pub(crate) struct OpenAIChatRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    pub max_tokens: usize,
    pub stream: bool,
}

/// A single role-tagged message in a chat request.
#[derive(Debug, Serialize)]
pub(crate) struct OpenAIMessage {
    pub role: String,
    pub content: String,
}
