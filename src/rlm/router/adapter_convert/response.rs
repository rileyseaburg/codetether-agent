//! Convert host completion responses into crate LLM responses.

use crate::provider::{CompletionResponse, ContentPart};
use codetether_rlm::traits::{LlmResponse, ToolCall};

pub(in crate::rlm::router) fn to_llm_resp(resp: CompletionResponse) -> LlmResponse {
    LlmResponse {
        text: text(&resp),
        tool_calls: tool_calls(&resp),
        finish_reason: Some(format!("{:?}", resp.finish_reason)),
        input_tokens: 0,
        output_tokens: 0,
    }
}

fn text(resp: &CompletionResponse) -> String {
    resp.message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect()
}

fn tool_calls(resp: &CompletionResponse) -> Vec<ToolCall> {
    resp.message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall { id, name, arguments, .. } => Some(ToolCall {
                id: id.clone(),
                name: name.clone(),
                arguments: serde_json::from_str(arguments).unwrap_or_default(),
            }),
            _ => None,
        })
        .collect()
}
