//! Provider adapter: main-crate `Provider` → crate `LlmProvider`.

use std::sync::Arc;
use codetether_rlm::traits::{LlmMessage, LlmProvider, LlmResponse, ToolDefinition};
use crate::provider::{CompletionRequest, CompletionResponse, ContentPart, Message, Provider, Role};

/// Wraps `Provider` as `LlmProvider`.
pub(super) struct ProviderWrap(pub(crate) Arc<dyn Provider>);

fn parse_role(s: &str) -> Role {
    match s { "user" => Role::User, "assistant" => Role::Assistant, "system" => Role::System, "tool" => Role::Tool, _ => Role::User }
}

fn build_req(msgs: Vec<LlmMessage>, tools: Vec<ToolDefinition>, model: &str, temp: Option<f32>) -> CompletionRequest {
    CompletionRequest {
        messages: msgs.into_iter().map(|m| Message { role: parse_role(&m.role), content: vec![ContentPart::Text { text: m.text }] }).collect(),
        tools: tools.into_iter().map(|t| crate::provider::ToolDefinition { name: t.name, description: t.description, parameters: t.parameters }).collect(),
        model: model.into(), temperature: temp, top_p: None, max_tokens: Some(4000), stop: vec![],
    }
}

fn to_llm_resp(resp: CompletionResponse) -> LlmResponse {
    let text: String = resp.message.content.iter().filter_map(|p| match p { ContentPart::Text { text } => Some(text.clone()), _ => None }).collect();
    let tc: Vec<codetether_rlm::traits::ToolCall> = resp.message.content.iter().filter_map(|p| match p { ContentPart::ToolCall { id, name, arguments, .. } => Some(codetether_rlm::traits::ToolCall { id: id.clone(), name: name.clone(), arguments: serde_json::from_str(arguments).unwrap_or_default() }), _ => None }).collect();
    LlmResponse { text, tool_calls: tc, finish_reason: resp.finish_reason.clone(), input_tokens: 0, output_tokens: 0 }
}

#[async_trait::async_trait]
impl LlmProvider for ProviderWrap {
    async fn complete(&self, messages: Vec<LlmMessage>, tools: Vec<ToolDefinition>, model: &str, temperature: Option<f32>) -> anyhow::Result<LlmResponse> {
        let req = build_req(messages, tools, model, temperature);
        let resp = self.0.complete(req).await?;
        Ok(to_llm_resp(resp))
    }
}
