//! OpenAI-compatible chat response types for the HTTP thinker backend.

use super::content::OpenAIChatContent;
use serde::Deserialize;

/// Chat-completions response body.
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIChatResponse {
    pub model: Option<String>,
    pub choices: Vec<OpenAIChatChoice>,
    #[serde(default)]
    pub usage: Option<OpenAIUsage>,
}

/// A single completion choice.
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIChatChoice {
    pub message: OpenAIChatChoiceMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

/// Assistant message, which may carry text or reasoning-only output.
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIChatChoiceMessage {
    #[serde(default)]
    content: Option<OpenAIChatContent>,
    #[serde(default)]
    reasoning: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

/// Token accounting reported by the server.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct OpenAIUsage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

impl OpenAIChatChoiceMessage {
    /// Extract assistant text, preferring content over reasoning fields.
    pub(crate) fn extract_text(&self) -> String {
        let content_text = self
            .content
            .as_ref()
            .map(OpenAIChatContent::to_text)
            .unwrap_or_default();
        if !content_text.trim().is_empty() {
            return content_text;
        }

        [self.reasoning.as_deref(), self.reasoning_content.as_deref()]
            .into_iter()
            .flatten()
            .find(|text| !text.trim().is_empty())
            .unwrap_or_default()
            .to_string()
    }
}
