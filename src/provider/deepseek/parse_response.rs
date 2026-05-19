//! Parse a DeepSeek API response into our internal types.

use crate::provider::{CompletionResponse, ContentPart, FinishReason, Message, Role, Usage};

use super::response::DsResponse;

pub(super) fn parse(ds: DsResponse) -> anyhow::Result<CompletionResponse> {
    let choice = ds
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices in DeepSeek response"))?;
    let mut content = Vec::new();
    let mut has_tool_calls = false;

    if let Some(ref r) = choice.message.reasoning_content {
        if !r.is_empty() {
            content.push(ContentPart::Thinking { text: r.clone() });
        }
    }
    if let Some(ref t) = choice.message.content {
        if !t.is_empty() {
            content.push(ContentPart::Text { text: t.clone() });
        }
    }
    if let Some(ref calls) = choice.message.tool_calls {
        has_tool_calls = !calls.is_empty();
        for tc in calls {
            content.push(ContentPart::ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                arguments: tc.function.arguments.clone(),
                thought_signature: None,
            });
        }
    }
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: Usage {
            prompt_tokens: ds.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
            completion_tokens: ds.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
            total_tokens: ds.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
            ..Default::default()
        },
        finish_reason: finish(&choice.finish_reason, has_tool_calls),
    })
}

fn finish(reason: &Option<String>, has_calls: bool) -> FinishReason {
    if has_calls {
        return FinishReason::ToolCalls;
    }
    match reason.as_deref() {
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCalls,
        _ => FinishReason::Stop,
    }
}
