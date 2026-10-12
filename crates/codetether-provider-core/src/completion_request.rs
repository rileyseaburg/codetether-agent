use crate::{Message, ToolDefinition};

/// Request to generate a completion.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::CompletionRequest;
/// let request = CompletionRequest {
///     messages: vec![], tools: vec![], model: "example-model".into(),
///     temperature: None, top_p: None, max_tokens: None, stop: vec![],
/// };
/// assert_eq!(request.model, "example-model");
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
