//! Core types shared across all AI providers.
//!
//! Defines the message, request, response, and streaming types that every
//! provider implementation must accept or produce.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::{Message, Role, ContentPart};
//!
//! let msg = Message {
//!     role: Role::User,
//!     content: vec![ContentPart::Text { text: "hello".into() }],
//! };
//! assert_eq!(msg.role, Role::User);
//! ```

use serde::{Deserialize, Serialize};

/// A message in a conversation.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{Message, Role, ContentPart};
/// let msg = Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hello".into() }],
/// };
/// assert_eq!(msg.role, Role::User);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Who sent this message.
    pub role: Role,
    /// Ordered content blocks (text, images, tool calls, etc.).
    pub content: Vec<ContentPart>,
}

/// Participant role in a conversation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System prompt / instructions.
    System,
    /// End-user input.
    User,
    /// Model response.
    Assistant,
    /// Tool result to be fed back to the model.
    Tool,
}

/// One content block within a [`Message`].
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::ContentPart;
/// let text = ContentPart::Text { text: "hi".into() };
/// let tool_result = ContentPart::ToolResult {
///     tool_call_id: "call_1".into(),
///     content: "ok".into(),
/// };
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
    Thinking { text: String },
}

/// Schema-driven tool definition passed to the model.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::ToolDefinition;
/// let tool = ToolDefinition {
///     name: "bash".into(),
///     description: "Run a shell command".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool function name (e.g. `"bash"`).
    pub name: String,
    /// Human-readable description the model uses to decide when to call.
    pub description: String,
    /// JSON Schema for the tool's input parameters.
    pub parameters: serde_json::Value,
}

/// Request to generate a completion.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{CompletionRequest, Message, Role, ContentPart};
/// let req = CompletionRequest {
///     messages: vec![Message {
///         role: Role::User,
///         content: vec![ContentPart::Text { text: "hello".into() }],
///     }],
///     tools: vec![],
///     model: "gpt-4o".into(),
///     temperature: None,
///     top_p: None,
///     max_tokens: None,
///     stop: vec![],
/// };
/// assert_eq!(req.model, "gpt-4o");
/// ```
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    /// Conversation history (system + user + assistant + tool).
    pub messages: Vec<Message>,
    /// Tools the model may invoke.
    pub tools: Vec<ToolDefinition>,
    /// Model identifier (provider-alias or full ID).
    pub model: String,
    /// Sampling temperature (0–2).
    pub temperature: Option<f32>,
    /// Nucleus sampling threshold.
    pub top_p: Option<f32>,
    /// Maximum tokens the model should generate.
    pub max_tokens: Option<usize>,
    /// Strings that cause the model to stop generating.
    pub stop: Vec<String>,
}

/// Request to generate embeddings.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::EmbeddingRequest;
/// let req = EmbeddingRequest {
///     model: "text-embedding-3-small".into(),
///     inputs: vec!["hello".into()],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct EmbeddingRequest {
    /// Embedding model identifier.
    pub model: String,
    /// Text inputs to embed.
    pub inputs: Vec<String>,
}

/// Response from an embedding request.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{EmbeddingResponse, Usage};
/// let resp = EmbeddingResponse {
///     embeddings: vec![vec![0.1, 0.2]],
///     usage: Usage::default(),
/// };
/// assert_eq!(resp.embeddings.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct EmbeddingResponse {
    /// One float vector per input.
    pub embeddings: Vec<Vec<f32>>,
    /// Token usage.
    pub usage: Usage,
}

/// A streaming chunk produced by [`Provider::complete_stream`](super::Provider::complete_stream).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::StreamChunk;
/// let chunk = StreamChunk::Text("hello".into());
/// assert!(matches!(chunk, StreamChunk::Text(_)));
/// ```
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Incremental text delta.
    Text(String),
    /// Model thinking/reasoning content.
    Thinking(String),
    /// Beginning of a tool call.
    ToolCallStart { id: String, name: String },
    /// Partial tool-call arguments.
    ToolCallDelta { id: String, arguments_delta: String },
    /// End of a tool call.
    ToolCallEnd { id: String },
    /// Stream finished.
    Done { usage: Option<Usage> },
    /// Recoverable error (stream continues).
    Error(String),
}

/// Token usage statistics.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::Usage;
/// let usage = Usage { prompt_tokens: 10, completion_tokens: 5, ..Usage::default() };
/// assert_eq!(usage.total_tokens, 0); // default
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    /// Tokens in the prompt.
    pub prompt_tokens: usize,
    /// Tokens in the completion.
    pub completion_tokens: usize,
    /// Total (prompt + completion).
    pub total_tokens: usize,
    /// Prompt-cache read hits (Anthropic / Bedrock).
    pub cache_read_tokens: Option<usize>,
    /// Prompt-cache write misses.
    pub cache_write_tokens: Option<usize>,
}

/// Response from a completion request.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{CompletionResponse, FinishReason, Message, Role, ContentPart, Usage};
/// let resp = CompletionResponse {
///     message: Message { role: Role::Assistant, content: vec![] },
///     usage: Usage::default(),
///     finish_reason: FinishReason::Stop,
/// };
/// assert_eq!(resp.finish_reason, FinishReason::Stop);
/// ```
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    /// The model's reply message.
    pub message: Message,
    /// Token usage for this request.
    pub usage: Usage,
    /// Why the model stopped generating.
    pub finish_reason: FinishReason,
}

/// Reason the model stopped generating.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// Natural stop or stop sequence hit.
    Stop,
    /// Hit the `max_tokens` limit.
    Length,
    /// The model requested tool execution.
    ToolCalls,
    /// Content was filtered by safety systems.
    ContentFilter,
    /// An error occurred.
    Error,
}
