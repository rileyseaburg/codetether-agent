use super::{ContextEvent, ContextTrace};

impl ContextTrace {
    /// Estimate token count from text using about four chars per token.
    pub fn estimate_tokens(text: &str) -> usize {
        (text.chars().count() / 4).max(1)
    }

    /// Create an event from text with automatic token estimation.
    pub fn event_from_text(event: ContextEvent, text: &str) -> ContextEvent {
        let tokens = Self::estimate_tokens(text);
        match event {
            ContextEvent::SystemPrompt { content, .. } => Self::system_prompt(content, tokens),
            ContextEvent::GrepResult {
                pattern, matches, ..
            } => Self::grep_result(pattern, matches, tokens),
            ContextEvent::LlmQueryResult {
                query,
                response_preview,
                ..
            } => Self::llm_result(query, response_preview, tokens),
            ContextEvent::AssistantCode { code, .. } => {
                ContextEvent::AssistantCode { code, tokens }
            }
            ContextEvent::ExecutionOutput { output, .. } => {
                ContextEvent::ExecutionOutput { output, tokens }
            }
            ContextEvent::Final { answer, .. } => ContextEvent::Final { answer, tokens },
            ContextEvent::ToolCall {
                name,
                arguments_preview,
                ..
            } => Self::tool_call(name, arguments_preview, tokens),
            ContextEvent::ToolResult {
                tool_call_id,
                result_preview,
                ..
            } => Self::tool_result(tool_call_id, result_preview, tokens),
        }
    }

    fn system_prompt(content: String, tokens: usize) -> ContextEvent {
        ContextEvent::SystemPrompt { content, tokens }
    }
}
