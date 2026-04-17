//! Bedrock Converse API response types and helpers.
//!
//! These types mirror the JSON shape returned from the `/converse` endpoint
//! and provide a small helper to translate it into the crate's generic
//! [`CompletionResponse`].
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::parse_converse_response;
//!
//! let json = serde_json::json!({
//!     "output": {"message": {"role": "assistant", "content": [{"text": "hi"}]}},
//!     "stopReason": "end_turn",
//!     "usage": {"inputTokens": 3, "outputTokens": 1, "totalTokens": 4}
//! });
//! let resp = parse_converse_response(&json.to_string()).unwrap();
//! assert_eq!(resp.usage.total_tokens, 4);
//! ```

use crate::provider::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, Usage,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseResponse {
    output: ConverseOutput,
    #[serde(default)]
    stop_reason: Option<String>,
    #[serde(default)]
    usage: Option<ConverseUsage>,
}

#[derive(Debug, Deserialize)]
struct ConverseOutput {
    message: ConverseMessage,
}

#[derive(Debug, Deserialize)]
struct ConverseMessage {
    #[allow(dead_code)]
    role: String,
    content: Vec<ConverseContent>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConverseContent {
    ReasoningContent {
        #[serde(rename = "reasoningContent")]
        reasoning_content: ReasoningContentBlock,
    },
    Text {
        text: String,
    },
    ToolUse {
        #[serde(rename = "toolUse")]
        tool_use: ConverseToolUse,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReasoningContentBlock {
    reasoning_text: ReasoningText,
}

#[derive(Debug, Deserialize)]
struct ReasoningText {
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseToolUse {
    tool_use_id: String,
    name: String,
    input: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseUsage {
    #[serde(default)]
    input_tokens: usize,
    #[serde(default)]
    output_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

/// Error body returned by Bedrock when a request is rejected.
#[derive(Debug, Deserialize)]
pub struct BedrockError {
    /// Human-readable error message.
    pub message: String,
}

/// Parse a Bedrock Converse API response JSON string into a
/// [`CompletionResponse`].
///
/// # Errors
///
/// Returns [`anyhow::Error`] if the string is not valid JSON in the expected
/// Converse response shape.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::parse_converse_response;
/// use codetether_agent::provider::{ContentPart, Role};
///
/// let body = r#"{"output":{"message":{"role":"assistant","content":[{"text":"hi"}]}},"stopReason":"end_turn"}"#;
/// let resp = parse_converse_response(body).unwrap();
/// assert!(matches!(resp.message.role, Role::Assistant));
/// assert!(matches!(&resp.message.content[0], ContentPart::Text { text } if text == "hi"));
/// ```
pub fn parse_converse_response(text: &str) -> Result<CompletionResponse> {
    let response: ConverseResponse = serde_json::from_str(text).context(format!(
        "Failed to parse Bedrock response: {}",
        crate::util::truncate_bytes_safe(text, 300)
    ))?;

    let mut content = Vec::new();
    let mut has_tool_calls = false;

    for part in &response.output.message.content {
        match part {
            ConverseContent::ReasoningContent { reasoning_content } => {
                if !reasoning_content.reasoning_text.text.is_empty() {
                    content.push(ContentPart::Thinking {
                        text: reasoning_content.reasoning_text.text.clone(),
                    });
                }
            }
            ConverseContent::Text { text } => {
                if !text.is_empty() {
                    content.push(ContentPart::Text { text: text.clone() });
                }
            }
            ConverseContent::ToolUse { tool_use } => {
                has_tool_calls = true;
                content.push(ContentPart::ToolCall {
                    id: tool_use.tool_use_id.clone(),
                    name: tool_use.name.clone(),
                    arguments: serde_json::to_string(&tool_use.input).unwrap_or_default(),
                    thought_signature: None,
                });
            }
        }
    }

    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop") | Some("stop_sequence") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::ToolCalls,
            Some("content_filtered") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };

    let usage = response.usage.as_ref();

    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: Usage {
            prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
            completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
            total_tokens: usage.map(|u| u.total_tokens).unwrap_or(0),
            cache_read_tokens: None,
            cache_write_tokens: None,
        },
        finish_reason,
    })
}
