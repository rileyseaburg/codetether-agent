use super::ContextEvent;

impl ContextEvent {
    /// Get the token count for this event.
    pub fn tokens(&self) -> usize {
        match self {
            Self::SystemPrompt { tokens, .. }
            | Self::GrepResult { tokens, .. }
            | Self::LlmQueryResult { tokens, .. }
            | Self::AssistantCode { tokens, .. }
            | Self::ExecutionOutput { tokens, .. }
            | Self::Final { tokens, .. }
            | Self::ToolCall { tokens, .. }
            | Self::ToolResult { tokens, .. } => *tokens,
        }
    }

    /// Get a human-readable label for this event type.
    pub fn label(&self) -> &'static str {
        match self {
            Self::SystemPrompt { .. } => "system_prompt",
            Self::GrepResult { .. } => "grep_result",
            Self::LlmQueryResult { .. } => "llm_query_result",
            Self::AssistantCode { .. } => "assistant_code",
            Self::ExecutionOutput { .. } => "execution_output",
            Self::Final { .. } => "final",
            Self::ToolCall { .. } => "tool_call",
            Self::ToolResult { .. } => "tool_result",
        }
    }
}
