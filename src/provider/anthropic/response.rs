//! Anthropic response payload types.
//!
//! This module defines the deserialization-only data structures that mirror
//! successful Anthropic Messages API responses. Conversion into the
//! application-wide provider model happens in the parsing modules, allowing
//! these types to stay close to the wire format returned by Anthropic and
//! Anthropic-compatible services.

use serde::Deserialize;
use serde_json::Value;

/// Successful Messages API response envelope.
///
/// Represents the top-level JSON object returned for a completed
/// Anthropic-compatible Messages API request. The response includes provider
/// identifiers, ordered content blocks, an optional stop reason, and optional
/// token usage accounting.
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicResponse {
    /// Provider-generated response identifier for tracing and diagnostics.
    pub(crate) id: String,
    /// Model identifier that handled the request.
    pub(crate) model: String,
    /// Ordered assistant content blocks emitted by the model.
    pub(crate) content: Vec<AnthropicContent>,
    /// Optional provider stop reason, such as `end_turn`, `max_tokens`, or `tool_use`.
    #[serde(default)]
    pub(crate) stop_reason: Option<String>,
    /// Optional token usage accounting returned with the response.
    #[serde(default)]
    pub(crate) usage: Option<AnthropicUsage>,
}

/// Content blocks emitted by Anthropic-compatible models.
///
/// Anthropic responses contain a sequence of typed content blocks rather than a
/// single text field. This enum captures the block types used by the provider
/// implementation: visible text, extended-thinking content, tool calls, and a
/// catch-all variant for unsupported block types.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum AnthropicContent {
    /// Visible assistant text returned to the user.
    #[serde(rename = "text")]
    Text { text: String },
    /// Extended-thinking content and its optional provider signature.
    #[serde(rename = "thinking")]
    Thinking {
        /// Primary reasoning text when supplied by the provider.
        #[serde(default)]
        thinking: Option<String>,
        /// Fallback reasoning text used by some Anthropic-compatible providers.
        #[serde(default)]
        text: Option<String>,
        /// Optional signature associated with the thinking block.
        #[serde(default)]
        signature: Option<String>,
    },
    /// Tool invocation requested by the model.
    #[serde(rename = "tool_use")]
    ToolUse {
        /// Provider-generated tool call identifier.
        id: String,
        /// Name of the tool the model requested.
        name: String,
        /// JSON arguments supplied for the tool call.
        input: Value,
    },
    /// Unsupported content block type ignored by response parsing.
    #[serde(other)]
    Unknown,
}

/// Token usage accounting returned by Anthropic-compatible APIs.
///
/// Usage values are optional or defaulted because compatible providers may omit
/// individual accounting fields. Prompt-cache counts are represented separately
/// so callers can distinguish ordinary input tokens from cache creation and
/// cache read activity.
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicUsage {
    /// Number of input tokens charged or processed for the request.
    #[serde(default)]
    pub(crate) input_tokens: usize,
    /// Number of output tokens generated in the response.
    #[serde(default)]
    pub(crate) output_tokens: usize,
    /// Number of input tokens written into the prompt cache, when reported.
    #[serde(default)]
    pub(crate) cache_creation_input_tokens: Option<usize>,
    /// Number of input tokens read from the prompt cache, when reported.
    #[serde(default)]
    pub(crate) cache_read_input_tokens: Option<usize>,
}
