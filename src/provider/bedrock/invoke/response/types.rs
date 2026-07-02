//! Deserialization types for native Anthropic Messages responses.

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(in crate::provider::bedrock) struct AnthropicMessagesResponse {
    pub content: Vec<AnthropicContent>,
    #[serde(default)]
    pub stop_reason: Option<String>,
    #[serde(default)]
    pub usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(in crate::provider::bedrock) enum AnthropicContent {
    Text {
        text: String,
    },
    Thinking {
        #[serde(default)]
        thinking: String,
        #[serde(default)]
        signature: Option<String>,
    },
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Default)]
pub(in crate::provider::bedrock) struct AnthropicUsage {
    #[serde(default)]
    pub input_tokens: usize,
    #[serde(default)]
    pub output_tokens: usize,
    #[serde(default)]
    pub cache_read_input_tokens: usize,
    #[serde(default)]
    pub cache_creation_input_tokens: usize,
}
