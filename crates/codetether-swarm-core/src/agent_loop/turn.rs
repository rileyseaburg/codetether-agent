use super::*;

/// Separate provider response text, thinking and tool calls in their original order.
///
/// # Arguments
/// * `response` - Completed provider response.
///
/// # Returns
/// A turn retaining the response message and finish reason.
///
/// # Examples
/// See the module overview for how this operation participates in an agent turn.
pub fn parse_turn(response: CompletionResponse) -> Turn {
    let mut text = Vec::new();
    let mut thinking = Vec::new();
    let mut tools = Vec::new();
    for part in &response.message.content {
        match part {
            ContentPart::Text { text: value } => text.push(value.clone()),
            ContentPart::Thinking { text: value, .. } if !value.is_empty() => {
                thinking.push(value.clone())
            }
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => tools.push(ToolCall {
                id: id.clone(),
                name: name.clone(),
                arguments: arguments.clone(),
            }),
            ContentPart::Thinking { .. }
            | ContentPart::ToolResult { .. }
            | ContentPart::Image { .. }
            | ContentPart::File { .. } => {}
        }
    }
    tracing::info!(finish_reason = ?response.finish_reason,
        prompt_tokens = response.usage.prompt_tokens,
        completion_tokens = response.usage.completion_tokens, "Sub-agent provider turn completed");
    Turn {
        message: response.message,
        finish: response.finish_reason,
        text: text.join("\n"),
        thinking: thinking.join("\n"),
        tools,
    }
}
