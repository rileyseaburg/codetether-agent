use serde::{Deserialize, Serialize};

/// One content block within a [`Message`](crate::Message).
///
/// Variants represent text, images, files, tool calls/results and reasoning.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::ContentPart;
/// let part = ContentPart::Text { text: "hi".into() };
/// match part {
///     ContentPart::Text { text } => assert_eq!(text, "hi"),
///     _ => panic!("expected text"),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    /// Plain text.
    Text { text: String },
    /// Image referenced by URL.
    Image {
        url: String,
        mime_type: Option<String>,
    },
    /// File attachment.
    File {
        path: String,
        mime_type: Option<String>,
    },
    /// A tool call made by the model.
    ToolCall {
        id: String,
        name: String,
        arguments: String,
        /// Thought signature for Gemini 3.x models.
        #[serde(skip_serializing_if = "Option::is_none")]
        thought_signature: Option<String>,
    },
    /// Tool execution result to return to the model.
    ToolResult {
        tool_call_id: String,
        content: String,
    },
    /// Extended thinking / reasoning output.
    Thinking {
        text: String,
        signature: Option<String>,
    },
}
