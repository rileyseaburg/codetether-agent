//! Convert RLM requests into shared provider completion requests.

use crate::traits::{LlmMessage, ToolDefinition};
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};

/// Convert RLM messages and tool schemas into a provider request.
///
/// # Arguments
/// * `msgs` - Ordered RLM messages with string roles.
/// * `tools` - Tool schemas exposed to the model.
/// * `model` - Provider-local model identifier.
/// * `temp` - Optional sampling temperature.
/// # Returns
/// A provider request with a 4,000-token output limit and no stop sequences.
/// # Examples
/// ```
/// let request = codetether_rlm::provider_adapter::build_req(vec![], vec![], "model", None);
/// assert_eq!(request.max_tokens, Some(4000));
/// ```
pub fn build_req(
    msgs: Vec<LlmMessage>,
    tools: Vec<ToolDefinition>,
    model: &str,
    temp: Option<f32>,
) -> CompletionRequest {
    CompletionRequest {
        messages: msgs
            .into_iter()
            .map(|m| Message {
                role: parse_role(&m.role),
                content: vec![ContentPart::Text { text: m.text }],
            })
            .collect(),
        tools: tools.into_iter().map(tool_def).collect(),
        model: model.into(),
        temperature: temp,
        top_p: None,
        max_tokens: Some(4000),
        stop: vec![],
    }
}

fn parse_role(s: &str) -> Role {
    match s {
        "user" => Role::User,
        "assistant" => Role::Assistant,
        "system" => Role::System,
        "developer" => Role::Developer,
        "tool" => Role::Tool,
        _ => Role::User,
    }
}

fn tool_def(t: ToolDefinition) -> codetether_provider_core::ToolDefinition {
    codetether_provider_core::ToolDefinition {
        name: t.name,
        description: t.description,
        parameters: t.parameters,
    }
}
