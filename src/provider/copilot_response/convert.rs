//! Convert Copilot wire responses into provider responses.

use super::types::{CopilotChoice, CopilotResponse};
use crate::provider::{CompletionResponse, ContentPart, Message, Role};
use anyhow::{Result, anyhow};

pub(in crate::provider) fn to_completion_response(
    response: CopilotResponse,
) -> Result<CompletionResponse> {
    if response.choices.is_empty() {
        return Err(anyhow!("No choices"));
    }
    let choices = response.choices.as_slice();
    let (content, has_tool_calls) = content_parts(choices);
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: super::usage::from_copilot(response.usage.as_ref()),
        finish_reason: super::finish::reason(&response.choices, has_tool_calls),
    })
}

fn content_parts(choices: &[CopilotChoice]) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut found = false;
    for choice in choices {
        push_text(choice, &mut content);
        for tc in choice.message.tool_calls.iter().flatten() {
            found = true;
            let args = tc.function.arguments.as_deref().unwrap_or("{}").trim();
            let args = if args.is_empty() { "{}" } else { args };
            content.push(ContentPart::ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                arguments: args.to_string(),
                thought_signature: None,
            });
        }
    }
    (content, found)
}

fn push_text(choice: &CopilotChoice, content: &mut Vec<ContentPart>) {
    if let Some(text) = &choice.message.content
        && !text.is_empty()
    {
        content.push(ContentPart::Text { text: text.clone() });
    }
}
