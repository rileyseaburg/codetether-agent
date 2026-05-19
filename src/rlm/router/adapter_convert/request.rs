//! Convert crate LLM requests into host completion requests.

use crate::provider::{CompletionRequest, ContentPart, Message, Role};
use codetether_rlm::traits::{LlmMessage, ToolDefinition};

pub(in crate::rlm::router) fn build_req(
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
        "tool" => Role::Tool,
        _ => Role::User,
    }
}

fn tool_def(t: ToolDefinition) -> crate::provider::ToolDefinition {
    crate::provider::ToolDefinition {
        name: t.name,
        description: t.description,
        parameters: t.parameters,
    }
}
