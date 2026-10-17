//! assistant message conversion.
use crate::*;
pub(super) fn push(msg: &Message, result: &mut Vec<ChatMessage>) {
    let content = super::text::content(msg);
    let tool_calls: Vec<ToolCall> = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some(ToolCall {
                id: id.clone(),
                r#type: "function".to_string(),
                function: ToolCallFunction {
                    name: name.clone(),
                    arguments: arguments.clone(),
                },
            }),
            _ => None,
        })
        .collect();

    result.push(ChatMessage {
        role: "assistant".to_string(),
        // StepFun requires content field to be present (even if empty) when tool_calls exist
        content: if content.is_empty() && !tool_calls.is_empty() {
            Some(String::new())
        } else if content.is_empty() {
            None
        } else {
            Some(content)
        },
        tool_calls: if tool_calls.is_empty() {
            None
        } else {
            Some(tool_calls)
        },
        tool_call_id: None,
    });
}
